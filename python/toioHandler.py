from pythonosc import osc_server
from pythonosc import udp_client
from pythonosc.dispatcher import Dispatcher
import threading

class toioClient:
  '''Creates a client to communicate with the AxLab Rust Toio Server.'''

  def __init__(self, ip="127.0.0.1", port=3334):
    self.ip = ip
    self.port = port
    self.client = udp_client.SimpleUDPClient(self.ip, self.port)

  def send(self, message: str, vals: list):
    '''Sends an OSC message, alongside a series of values to the Toio Server'''
    self.client.send_message(message, vals)

  def motorBasic(self, cubeId: int, leftforwards: bool, leftspeed: int, rightforwards:bool, rightspeed: int):
    '''Activates the motors of a Toio. Specifications can be found at https://toio.github.io/toio-spec/en/docs/ble_motor#motor-control'''
    left = 1 if leftforwards else 2
    right = 1 if rightforwards else 2

    self.send("/motorBasic", [cubeId, left, leftspeed, right, rightspeed])

  def motorTarget(self, cubeId: int, mode: int, x: int, y: int, theta: int):
    '''Moves a Toio to a specific Target. Specifications can be found at https://toio.github.io/toio-spec/en/docs/ble_motor#motor-control-with-target-specified'''
    self.send("/motorTarget", [cubeId, mode, x, y, theta])

  def motorAcceleration(self, cubeId, speed, a, rotateVelocity, rotateDir, dir, priority, duration):
    '''Controls the accelration of a Toio. Specifications can be found at https://toio.github.io/toio-spec/en/docs/ble_motor#motor-control-with-target-specified'''
    self.send("/motorAcceleration", [cubeId, speed, a, rotateVelocity, rotateDir, dir, priority, duration])

  def led(self, cubeId, duration, red, green, blue):
    '''Control the color of the indicator LED of a Toio. Specifications can be found at https://toio.github.io/toio-spec/en/docs/ble_light'''
    self.send("/led", [cubeId, duration, red, green, blue])

  def sound(self, cubeId, soundeffect, volume):
    '''Triggers one of the predefined sound effects of a Toio. Specifications can be found at https://toio.github.io/toio-spec/en/docs/ble_light'''
    self.send("/sound", [cubeId, soundeffect, volume])

  def midi(self, cubeId: int, duration: int, noteID: int, volume: int):
    '''Plays a MIDI note on a Toio. Specifications can be found at https://toio.github.io/toio-spec/en/docs/ble_sound#playing-the-midi-note-numbers'''
    self.send("/midi", [cubeId, duration, noteID, volume])

  def motionRequest(self, cubeID: int):
    '''Sends a request for motion request information. Specifications can be found at https://toio.github.io/toio-spec/en/docs/ble_sensor'''
    self.send("/motion", [cubeID])

  def magneticRequest(self, cubeId: int):
    '''Sends a request for magnetic request information. Specifications can be found at https://toio.github.io/toio-spec/en/docs/ble_magnetic_sensor'''
    self.send("/magnetic", [cubeId])

  def postureRequest(self, cubeId: int, euler: bool):
    '''Sends a request for posture request information in eulers or quaternions. Specifications can be found at https://toio.github.io/toio-spec/en/docs/ble_high_precision_tilt_sensor'''
    if (euler): self.send("/postureeuler", [cubeId])
    else: self.send("/posturequaternion", [cubeId])

class toioServer:
  def __init__(self, ip="127.0.0.1", port=3333):
    dispatcher = Dispatcher()
    dispatcher.map("/position", print)
    dispatcher.map("/button", print)
    dispatcher.map("/motion", print)
    dispatcher.map("/magnetic", print)
    dispatcher.map("/postureeuler", print)
    dispatcher.map("/posturequaternion", print)
    dispatcher.map("/battery", print)

    self.ip = ip
    self.port = port
    self.server = osc_server.ThreadingOSCUDPServer((self.ip, self.port), dispatcher)
    self.thread = threading.Thread(target=self.server.serve_forever, args=(1,))
    self.thread.start()
