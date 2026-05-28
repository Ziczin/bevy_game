import os
from PIL import Image

for file in os.listdir('.'):
    if file.lower().endswith(('.png', '.jpg', '.jpeg', '.bmp', '.tiff', '.webp')):
        name, ext = os.path.splitext(file)
        new_file = f"{name}_new{ext}"
        try:
            img = Image.open(file)
            if img.size != (64, 64):
                continue
            if img.mode == 'RGBA':
                new_img = Image.new('RGBA', (16, 16))
            else:
                new_img = Image.new('RGB', (16, 16))
            orig_pixels = img.load()
            new_pixels = new_img.load()
            for i in range(16):
                for j in range(16):
                    new_pixels[i, j] = orig_pixels[i * 4, j * 4]
            new_img.save(new_file)
        except Exception:
            pass
