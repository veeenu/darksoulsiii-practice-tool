use hudhook::tracing::error;
use hudhook::RenderContext;
use imgui::{StyleColor, StyleVar, TextureId, Ui};

/// Side of each icon in the atlas, in pixels.
const SIZE: usize = 32;

/// Icons in atlas order.
const ICONS: [Icon; 4] = [Icon::Settings, Icon::Help, Icon::Trash, Icon::Grip];

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Icon {
    Settings,
    Help,
    Trash,
    Grip,
}

impl Icon {
    /// Text shown in place of the icon if the atlas couldn't be loaded.
    fn fallback(self) -> &'static str {
        match self {
            Icon::Settings => "Settings",
            Icon::Help => "Help",
            Icon::Trash => "Delete",
            Icon::Grip => "Move",
        }
    }

    /// Signed distance from the icon's shape, in `[-1, 1]` coordinates with y
    /// pointing down.
    fn sdf(self, p: [f32; 2]) -> f32 {
        match self {
            Icon::Settings => {
                let teeth = (0..8)
                    .map(|i| {
                        // Rotate each tooth's frame onto the x axis.
                        let (sin, cos) = (i as f32 * std::f32::consts::FRAC_PI_4).sin_cos();
                        let q = [p[0] * cos + p[1] * sin, p[1] * cos - p[0] * sin];
                        rect(q, [0.68, 0.], [0.2, 0.13])
                    })
                    .fold(f32::MAX, f32::min);
                let body = circle(p, [0., 0.], 0.6);
                let hole = circle(p, [0., 0.], 0.24);
                body.min(teeth).max(-hole)
            },
            Icon::Help => {
                let ring = circle(p, [0., 0.], 0.82).abs() - 0.08;
                let hook = polyline(
                    p,
                    &[
                        [-0.28, -0.22],
                        [-0.18, -0.42],
                        [0., -0.48],
                        [0.18, -0.42],
                        [0.27, -0.24],
                        [0.15, -0.06],
                        [0., 0.04],
                        [0., 0.2],
                    ],
                    0.09,
                );
                let dot = circle(p, [0., 0.44], 0.1);
                ring.min(hook).min(dot)
            },
            Icon::Trash => {
                let lid = capsule(p, [-0.7, -0.5], [0.7, -0.5], 0.08);
                let handle = polyline(
                    p,
                    &[[-0.22, -0.5], [-0.22, -0.72], [0.22, -0.72], [0.22, -0.5]],
                    0.07,
                );
                let body =
                    polyline(p, &[[-0.52, -0.3], [-0.42, 0.78], [0.42, 0.78], [0.52, -0.3]], 0.08);
                let lines = capsule(p, [-0.16, -0.1], [-0.13, 0.55], 0.06).min(capsule(
                    p,
                    [0.16, -0.1],
                    [0.13, 0.55],
                    0.06,
                ));
                lid.min(handle).min(body).min(lines)
            },
            Icon::Grip => [-0.5, 0., 0.5]
                .into_iter()
                .flat_map(|y| [circle(p, [-0.25, y], 0.14), circle(p, [0.25, y], 0.14)])
                .fold(f32::MAX, f32::min),
        }
    }
}

fn circle(p: [f32; 2], c: [f32; 2], r: f32) -> f32 {
    (p[0] - c[0]).hypot(p[1] - c[1]) - r
}

fn rect(p: [f32; 2], c: [f32; 2], half_size: [f32; 2]) -> f32 {
    let d = [(p[0] - c[0]).abs() - half_size[0], (p[1] - c[1]).abs() - half_size[1]];
    d[0].max(0.).hypot(d[1].max(0.)) + d[0].max(d[1]).min(0.)
}

fn capsule(p: [f32; 2], a: [f32; 2], b: [f32; 2], r: f32) -> f32 {
    let pa = [p[0] - a[0], p[1] - a[1]];
    let ba = [b[0] - a[0], b[1] - a[1]];
    let h = ((pa[0] * ba[0] + pa[1] * ba[1]) / (ba[0] * ba[0] + ba[1] * ba[1])).clamp(0., 1.);
    (pa[0] - ba[0] * h).hypot(pa[1] - ba[1] * h) - r
}

fn polyline(p: [f32; 2], points: &[[f32; 2]], r: f32) -> f32 {
    points.windows(2).map(|w| capsule(p, w[0], w[1], r)).fold(f32::MAX, f32::min)
}

/// Texture atlas of white, antialiased icons, tinted with the text color when
/// drawn.
#[derive(Default)]
pub(crate) struct Icons(Option<TextureId>);

impl Icons {
    pub(crate) fn load(render_context: &mut dyn RenderContext) -> Self {
        let width = ICONS.len() * SIZE;
        let mut data = vec![0u8; width * SIZE * 4];

        for (i, icon) in ICONS.into_iter().enumerate() {
            for y in 0..SIZE {
                for x in 0..SIZE {
                    let p = [x, y].map(|c| (c as f32 + 0.5) / SIZE as f32 * 2. - 1.);
                    let alpha = (0.5 - icon.sdf(p) * SIZE as f32 / 2.).clamp(0., 1.);
                    let offset = (y * width + i * SIZE + x) * 4;
                    data[offset..offset + 4].copy_from_slice(&[
                        255,
                        255,
                        255,
                        (alpha * 255.) as u8,
                    ]);
                }
            }
        }

        match render_context.load_texture(&data, width as u32, SIZE as u32) {
            Ok(texture) => Icons(Some(texture)),
            Err(e) => {
                error!("Couldn't load icons: {e:?}");
                Icons(None)
            },
        }
    }

    /// Square icon button, as high as other framed widgets.
    pub(crate) fn button(&self, ui: &Ui, id: &str, icon: Icon) -> bool {
        let padding = ui.clone_style().frame_padding[1];
        self.button_with_padding(ui, id, icon, padding)
    }

    /// Square icon button, as high as text.
    pub(crate) fn small_button(&self, ui: &Ui, id: &str, icon: Icon) -> bool {
        self.button_with_padding(ui, id, icon, 0.)
    }

    fn button_with_padding(&self, ui: &Ui, id: &str, icon: Icon, padding: f32) -> bool {
        let _padding = ui.push_style_var(StyleVar::FramePadding([padding, padding]));
        let Some(texture) = self.0 else {
            return ui.button(icon.fallback());
        };

        let index = ICONS.iter().position(|&i| i == icon).unwrap_or_default() as f32;
        let count = ICONS.len() as f32;
        let size = ui.current_font_size();

        ui.image_button_config(id, texture, [size, size])
            .uv0([index / count, 0.])
            .uv1([(index + 1.) / count, 1.])
            .tint_col(ui.style_color(StyleColor::Text))
            .build()
    }
}
