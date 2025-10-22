import sharp from 'sharp';
import { readFileSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const projectRoot = join(__dirname, '..');

async function generateIcons() {
  const svgPath = join(projectRoot, 'public/assets/sentinel-logo.svg');
  const iconPath = join(projectRoot, 'app-icon.png');

  console.log('Reading SVG from:', svgPath);
  const svgBuffer = readFileSync(svgPath);

  console.log('Generating 1024x1024 PNG for Tauri icon generation...');
  await sharp(svgBuffer)
    .resize(1024, 1024, {
      fit: 'contain',
      background: { r: 0, g: 0, b: 0, alpha: 0 }
    })
    .png()
    .toFile(iconPath);

  console.log('✅ Generated app-icon.png successfully!');
  console.log('Now run: pnpm tauri icon');
}

generateIcons().catch(console.error);
