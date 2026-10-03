# Tests

Use this file as a guideline for testing the various features in the practice tool.

# Versions

- [ ] 1.01.1
- [ ] 1.03.1
- [ ] 1.03.2
- [ ] 1.04.1
- [ ] 1.04.2
- [ ] 1.04.3
- [ ] 1.05.0
- [ ] 1.05.1
- [ ] 1.06.0
- [ ] 1.07.0
- [ ] 1.08.0
- [ ] 1.09.0
- [ ] 1.10.0
- [ ] 1.11.0
- [ ] 1.12.0
- [ ] 1.13.0
- [ ] 1.14.0
- [ ] 1.15.0
- [ ] 1.15.1
- [ ] 1.15.2

# Test tasks

## Startup

- [ ] Installed as `dinput8.dll`: holding right shift at startup starts the tool.
- [ ] Installed as `dinput8.dll`: running the exe starts the copy the game loaded.
- [ ] Running the exe from another folder injects its own DLL.
- [ ] Running the exe twice reports that the tool is already running.
- [ ] Unsupported game version: a message box explains it.

## Flags

- [ ] All no damage: neither you nor enemies should take damage.
- [ ] No death: you and enemies take damage, but you don't die.
- [ ] One shot: you kill enemies in one hit.
- [ ] Inf Stamina: stamina doesn't deplete when attacking/running during a fight.
- [ ] Inf Focus: focus doesn't deplete when using weapon arts or spells.
- [ ] Inf Consumables: consumable count doesn't go down when used.
- [ ] Deathcam: should toggle the top-down camera.
- [ ] Event draw / Stable/Bloodstain draw: debug spheres should appear/disappear.
- [ ] Event disable: events stop running.
- [ ] AI disable: enemies start/stop their AI.
- [ ] Ember: you become embered.
- [ ] Render characters/objects/map: self-descriptive.
- [ ] Collision mesh hi/lo/hit, Hurtbox, Debug draw, All draw hit, IK foot ray, Debug spheres:
  should appear/disappear.
- [ ] No Gravity: you are able to walk on air.
- [ ] No Collision: you should fall below the floor.
- [ ] Multi flag: toggles all of its flags at once.

## Widgets

- [ ] Savefile manager: should load the selected savefile.
- [ ] Item spawn: should spawn the desired item.
- [ ] Item spawn: should apply upgrades and infusion correctly to the spawned item.
- [ ] Item spawn: should provide the specified amount of items.
- [ ] Item spawn: should filter correctly, and show the item icons.
- [ ] Edit stats: should apply the correct stats and not crash for each stat.
- [ ] Save position: should change the numbers on 2nd line when saving.
- [ ] Load position: should reposition and reorient character.
- [ ] Nudge: should change character's height.
- [ ] Speed: should change character's animation speed.
- [ ] Color: should cycle the collision mesh colors.
- [ ] Add souls: self-descriptive.
- [ ] Open menu: should open the travel/attune menu.
- [ ] Quitout: should exit to main menu.
- [ ] Target: should show the info of the targeted enemy.
- [ ] Input viewer: should show controller and keyboard input.
- [ ] Group: should open a submenu with its widgets.
- [ ] Radial menu: opens with the configured combo, blocks the controller while it's open, and
  triggers the selected entry.
- [ ] Long menus scroll instead of overflowing the screen.

## Indicators

- [ ] Game version, IGT, FPS, Animation, ImGui debug info: should show.
- [ ] Player position, Player velocity: should update as you move.
- [ ] Player distance: should reset to zero with "Start XYZ", and grow as you move away.
- [ ] Frame counter: should reset with "Reset".

## Settings

- [ ] Config editor: change a hotkey and an indicator, reorder and delete an entry, then save and
  see the widgets reload.
- [ ] Config editor: an invalid edit shows an error with its key path.
- [ ] Update: the update button appears in the overlay when an update is available.
- [ ] `disable_update_prompt = true`: no update button appears.
