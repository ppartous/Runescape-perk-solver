from PIL import Image

image = Image.open("./images/icon.png")
open("./images/icon.bin", "wb").write(bytearray(image.convert('RGBA').tobytes()))
