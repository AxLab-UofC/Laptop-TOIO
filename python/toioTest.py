from toioHandler import toioClient, toioServer
import time

client = toioClient()
server = toioServer()
client.midi(0, 10, 64, 255)
time.sleep(0.5)
client.midi(0, 10, 63, 255)
time.sleep(0.5)
client.midi(0, 10, 64, 255)
time.sleep(0.5)
client.midi(0, 10, 63, 255)
time.sleep(0.5)
client.midi(0, 10, 64, 255)
time.sleep(0.5)
client.midi(0, 10, 63, 255)
time.sleep(0.5)
client.midi(0, 10, 59, 255)
time.sleep(0.5)
client.midi(0, 10, 62, 255)
time.sleep(0.5)
client.midi(0, 10, 60, 255)
time.sleep(0.5)
client.midi(0, 10, 57, 255)
time.sleep(0.5)


