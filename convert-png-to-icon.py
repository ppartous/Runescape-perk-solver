from PIL import Image

open("./images/icon.bin",
     "wb").write(bytearray(Image.open("./images/icon.png").convert('RGBA').tobytes()))
open("./images/icon.ico",
     "wb").write(Image.open("./images/icon.png").save("./images/icon.ico", format="ico"))
