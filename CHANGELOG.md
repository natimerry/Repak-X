# Repak X — Changelog

## [1.3.0](https://github.com/XzantGaming/Repak-X/releases/latest)


### 🔧 Backend / Logic
- Fixed an issue where installing a mod bundle with the same name as a disabled one would cause corruption
- Character IDs listing logic is now more robust
- Fixed an issue causing mods to be incorrectly flagged as 'Encrypted'
- Fixed Extraction of Materials
- Weird Quirky behaviour when initiating a mod install using the folder method instead of a Pak where MipMapped textures would lose their ubulk file causing validation issue thus causing the game to use its original mipmaps causing a weird Fade-In/Out effect to happen. Fixed to not happen anymore
- Fixed assets formatting to correctly show in Fmodel's viewer (most noticeable with materials)

### 🎨 Frontend / UI
- Tag system improvements
- Show mods from subfolders now available as an option in the app settings
- Fixed an issue where some settings would not save properly
- Filters, context menu, and details view improvements
