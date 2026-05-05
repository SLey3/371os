import numpy as np
from PIL import Image
import requests
from io import BytesIO
import sys
import os

WIDTH, HEIGHT = 80, 25
IMG_URL = sys.argv[1] if len(sys.argv) > 1 else "https://cd-rs.github.io/os/img/rainbow.jpg"


script_dir = os.path.dirname(__file__)
dump_path = os.path.join(script_dir, "dump.ppm")
img_rs_path = os.path.join(script_dir, "src", "img.rs")
dump = np.array(Image.open(dump_path))
vga_palette = dump[200, ::45, :3].astype(np.int32)


resp = requests.get(IMG_URL)
img = Image.open(BytesIO(resp.content)).convert("RGB")
img = img.resize((WIDTH, HEIGHT))
img_arr = np.array(img).astype(np.int32)


def get_nearest_vga(pixel):
    distances = np.sqrt(np.sum((vga_palette - pixel)**2, axis=1))
    return np.argmin(distances)

vga_mapped = [get_nearest_vga(pixel) << 4 for pixel in img_arr.reshape(-1, 3)]


with open(img_rs_path, "w") as f:
    array_str = ", ".join(map(str, vga_mapped))
    f.write(f"pub const ARR: [u8; {WIDTH * HEIGHT}] = [{array_str}];")