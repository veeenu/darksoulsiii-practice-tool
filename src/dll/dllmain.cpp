#define UNICODE
#include <MinHook.h>
#include <Windows.h>
#include <d3d11.h>
#include <dxgi.h>
#include <fstream>
#include <imgui.h>
#include <iostream>
#include <string>
#include <thread>
#include "imgui/imgui_impl_dx11.h"
#include "imgui/imgui_impl_win32.h"
#include "ui.h"
#include "config.h"

#ifndef BUILD_CONFIG
#define BUILD_CONFIG "RelWithDebInfo"
#endif

constexpr int c_strcmp( char const* lhs, char const* rhs ) {
    return (('\0' == lhs[0]) && ('\0' == rhs[0])) ? 0
        :  (lhs[0] != rhs[0]) ? (lhs[0] - rhs[0])
        : c_strcmp( lhs+1, rhs+1 );
}

typedef HRESULT(__fastcall *IDXGISwapChainPresent)(IDXGISwapChain *pSwapChain,
                                                   UINT SyncInterval,
                                                   UINT Flags);
IDXGISwapChainPresent presentTrampoline;

extern LRESULT ImGui_ImplWin32_WndProcHandler(HWND hWnd, UINT msg,
                                              WPARAM wParam, LPARAM lParam);

HRESULT GetDeviceAndCtxFromSwapchain(IDXGISwapChain *pSwapChain,
                                     ID3D11Device **ppDevice,
                                     ID3D11DeviceContext **ppContext) {
  HRESULT ret =
      pSwapChain->GetDevice(__uuidof(ID3D11Device), (PVOID *)ppDevice);

  if (SUCCEEDED(ret))
    (*ppDevice)->GetImmediateContext(ppContext);

  return ret;
}

BOOL initialized = false;
ID3D11DeviceContext *pContext = nullptr;
ID3D11Device *pDevice = NULL;
ID3D11RenderTargetView *mainRenderTargetView;
static IDXGISwapChain *pSwapChain = NULL;
static WNDPROC OriginalWndProcHandler = nullptr;
HWND window = nullptr;
long i = 0;

LRESULT CALLBACK hWndProc(HWND hWnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
  ImGuiIO &io = ImGui::GetIO();
  POINT mPos;
  GetCursorPos(&mPos);
  ScreenToClient(window, &mPos);
  ImGui::GetIO().MousePos.x = mPos.x;
  ImGui::GetIO().MousePos.y = mPos.y;

  ImGui_ImplWin32_WndProcHandler(hWnd, uMsg, wParam, lParam);

  return CallWindowProc(OriginalWndProcHandler, hWnd, uMsg, wParam, lParam);
}

HRESULT __fastcall PresentImpl(IDXGISwapChain *pChain, UINT SyncInterval, UINT Flags) {
  if (!initialized) {
    std::cout << "Initializing DirectX" << std::endl;
    if (FAILED(GetDeviceAndCtxFromSwapchain(pChain, &pDevice, &pContext))) {
      std::cout << "GetDeviceAndCtxFromSwapChain failed" << std::endl;
      return presentTrampoline(pChain, SyncInterval, Flags);
    }

    ImGui::CreateContext();
    ImGuiIO &io = ImGui::GetIO();

    DXGI_SWAP_CHAIN_DESC sd;
    pChain->GetDesc(&sd);
    io.ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard;
    io.IniFilename = nullptr;

    window = sd.OutputWindow;
    OriginalWndProcHandler =
        (WNDPROC)SetWindowLongPtr(window, GWLP_WNDPROC, (LONG_PTR)hWndProc);

    ImGui_ImplWin32_Init(window);
    ImGui_ImplDX11_Init(pDevice, pContext);
    ImGui::GetIO().ImeWindowHandle = window;

    ID3D11Texture2D *pBackBuffer;

    pChain->GetBuffer(0, __uuidof(ID3D11Texture2D), (LPVOID *)&pBackBuffer);
    pDevice->CreateRenderTargetView(pBackBuffer, NULL, &mainRenderTargetView);
    pBackBuffer->Release();

    initialized = true;
    std::cout << "DirectX initialized successfully" << std::endl;
  }

  ImGui_ImplWin32_NewFrame();
  ImGui_ImplDX11_NewFrame();

  ImGui::NewFrame();
  UI::Instance().Render();
  ImGui::EndFrame();
  ImGui::Render();

  pContext->OMSetRenderTargets(1, &mainRenderTargetView, NULL);
  ImGui_ImplDX11_RenderDrawData(ImGui::GetDrawData());

  return presentTrampoline(pChain, SyncInterval, Flags);
}

LPVOID swapchain_present_vtable_lookup() {
  // Credits: https://www.unknowncheats.me/forum/d3d-tutorials-and-source/88369-universal-d3d11-hook.html
  D3D_FEATURE_LEVEL featureLevel = D3D_FEATURE_LEVEL_11_0;
  ID3D11Device *pDevice = nullptr;
  ID3D11DeviceContext *pContext = nullptr;
  IDXGISwapChain* pSwapChain = nullptr;

  DXGI_SWAP_CHAIN_DESC swapChainDesc;
  ZeroMemory(&swapChainDesc, sizeof(swapChainDesc));
  swapChainDesc.BufferCount = 1;
  swapChainDesc.BufferDesc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
  swapChainDesc.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
  swapChainDesc.OutputWindow = GetForegroundWindow();
  swapChainDesc.SampleDesc.Count = 1;
  swapChainDesc.Windowed = TRUE;
  swapChainDesc.BufferDesc.ScanlineOrdering = DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED;
  swapChainDesc.BufferDesc.Scaling = DXGI_MODE_SCALING_UNSPECIFIED;
  swapChainDesc.SwapEffect = DXGI_SWAP_EFFECT_DISCARD;
  if (FAILED(D3D11CreateDeviceAndSwapChain(
      NULL, D3D_DRIVER_TYPE_HARDWARE, NULL, NULL, &featureLevel, 1,
      D3D11_SDK_VERSION, &swapChainDesc, &pSwapChain, &pDevice, NULL, &pContext))) {
    std::cout << "D3D11CreateDeviceAndSwapChain failed" << std::endl;
    return nullptr;
  }

  DWORD_PTR* pSwapChainVtable;
  pSwapChainVtable = (DWORD_PTR*)pSwapChain;
  pSwapChainVtable = (DWORD_PTR*)pSwapChainVtable[0];
  LPVOID ret = reinterpret_cast<LPVOID>(pSwapChainVtable[8]);

  pDevice->Release();
  pContext->Release();
  pSwapChain->Release();

  return ret;
}

DWORD WINAPI run_thread(LPVOID param) {
  if constexpr(c_strcmp(BUILD_CONFIG, "RelWithDebInfo") == 0) {
    AllocConsole();
    SetConsoleTitle(L"Practice Tool DLL by johndisandonato");
    freopen("CONOUT$", "w", stdout);
    freopen("CONOUT$", "w", stderr);
    freopen("CONIN$", "r", stdin);
  }

  auto cfg = Config::Instance();
  auto s_true = std::string("true");
  std::cout << "Setting: " << cfg.setting("enabled") << std::endl;
  if (cfg.setting("enabled") != s_true) {
    return 0;
  }

  std::cout << "Hooking functions..." << std::endl;

  DWORD_PTR hDxgi = (DWORD_PTR)GetModuleHandle(L"dxgi.dll");

  LPVOID presentOriginal = swapchain_present_vtable_lookup();

  MH_Initialize();
  MH_CreateHook(presentOriginal, &PresentImpl,
                reinterpret_cast<LPVOID *>(&presentTrampoline));
  MH_EnableHook(presentOriginal);

  return 0;
}

BOOL APIENTRY DllMain(HMODULE hModule, DWORD ul_reason_for_call,
                      LPVOID lpReserved) {
  switch (ul_reason_for_call) {
    case DLL_PROCESS_ATTACH:
      DWORD tmp;
      CreateThread(NULL, 0, run_thread, NULL, 0, &tmp);
      break;
  }
  return TRUE;
}
