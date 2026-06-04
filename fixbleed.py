import sys
from pathlib import Path
import numpy as np
from PIL import Image

def process_image(input_path):
    img = Image.open(input_path).convert('RGBA')
    img_array = np.array(img)
    
    height, width, channels = img_array.shape
    
    if width % 16 != 0 or height % 16 != 0:
        print(f"Error {input_path}")
        return
    
    tiles_x = width // 16
    tiles_y = height // 16
    n_tiles = tiles_x * tiles_y
    
    new_width = tiles_x * 18
    new_height = tiles_y * 18
    
    new_array = np.zeros((new_height, new_width, channels), dtype=img_array.dtype)
    
    for ty in range(tiles_y):
        for tx in range(tiles_x):
            src_y0 = ty * 16
            src_x0 = tx * 16
            dst_y0 = ty * 18
            dst_x0 = tx * 18
            
            new_array[dst_y0+1:dst_y0+17, dst_x0+1:dst_x0+17] = img_array[src_y0:src_y0+16, src_x0:src_x0+16]
            new_array[dst_y0, dst_x0+1:dst_x0+17] = img_array[src_y0, src_x0:src_x0+16]
            new_array[dst_y0+17, dst_x0+1:dst_x0+17] = img_array[src_y0+15, src_x0:src_x0+16]
            new_array[dst_y0+1:dst_y0+17, dst_x0] = img_array[src_y0:src_y0+16, src_x0]
            new_array[dst_y0+1:dst_y0+17, dst_x0+17] = img_array[src_y0:src_y0+16, src_x0+15]
            
            new_array[dst_y0, dst_x0] = img_array[src_y0, src_x0]
            new_array[dst_y0, dst_x0+17] = img_array[src_y0, src_x0+15]
            new_array[dst_y0+17, dst_x0] = img_array[src_y0+15, src_x0]
            new_array[dst_y0+17, dst_x0+17] = img_array[src_y0+15, src_x0+15]
            
    output_path = input_path.with_name(f"{input_path.stem}_withpad.png")
    Image.fromarray(new_array).save(output_path, 'PNG')
    print(f"Success {input_path} with {n_tiles} tiles")

def find_images(root_dir):
    images = []
    for path in Path(root_dir).rglob('*.png'):
        if '_withpad' not in path.name:
            images.append(path)
    return images

def main():
    root_dir = sys.argv[1] if len(sys.argv) > 1 else '.'
    images = find_images(root_dir)
    for img_path in images:
        process_image(img_path)

if __name__ == '__main__':
    main()