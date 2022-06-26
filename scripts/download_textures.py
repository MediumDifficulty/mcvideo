import os
import requests

textures = [
    'black_concrete.png',
	'black_glazed_terracotta.png',
	'black_wool.png',
	'blue_concrete.png',
	'blue_glazed_terracotta.png',
	'blue_wool.png',
	'brown_concrete.png',
	'brown_glazed_terracotta.png',
	'brown_wool.png',
	'cyan_concrete.png',
	'cyan_glazed_terracotta.png',
	'cyan_wool.png',
	'gray_concrete.png',
	'gray_glazed_terracotta.png',
	'gray_wool.png',
	'green_concrete.png',
	'green_glazed_terracotta.png',
	'green_wool.png',
	'light_blue_concrete.png',
	'light_blue_glazed_terracotta.png',
	'light_blue_wool.png',
	'light_gray_concrete.png',
	'light_gray_glazed_terracotta.png',
	'light_gray_wool.png',
	'lime_concrete.png',
	'lime_glazed_terracotta.png',
	'lime_wool.png',
	'magenta_concrete.png',
	'magenta_glazed_terracotta.png',
	'magenta_wool.png',
	'orange_concrete.png',
	'orange_glazed_terracotta.png',
	'orange_wool.png',
	'pink_concrete.png',
	'pink_glazed_terracotta.png',
	'pink_wool.png',
	'purple_concrete.png',
	'purple_glazed_terracotta.png',
	'purple_wool.png',
	'red_concrete.png',
	'red_glazed_terracotta.png',
	'red_wool.png',
	'white_concrete.png',
	'white_glazed_terracotta.png',
	'white_wool.png',
	'yellow_concrete.png',
	'yellow_glazed_terracotta.png',
	'yellow_wool.png'
]


textures_path = '../textures'

if not os.path.exists(textures_path):
    os.mkdir('../textures')

for texture in textures:
    url = "https://raw.githubusercontent.com/InventivetalentDev/minecraft-assets/1.19/assets/minecraft/textures/block/" + texture
    file = requests.get(url).content

    open(textures_path + '/' + texture, 'wb').write(file)

    print("Downloaded: " + texture)