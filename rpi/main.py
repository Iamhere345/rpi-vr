# from sense_emu import SenseHat
import time
import socket

#! i couldn't get sense_emu working on windows. I might have another go on MacOS, but for now i may have to leave the proper testing
#! until i boot up the raspberry pi again. I've temporarily commented out SenseHAT-related code, and instead of emulator gryo data
#! i'm using constant test values

UPDATE_INTERVAL = 0.5

UDP_IP = "127.0.0.1"
UDP_PORT = 8080

# SenseHAT init
# sense = SenseHat()
# sense.clear()

# UDP init
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

while True:
  
  # o = sense.get_orientation()
  # pitch = o["pitch"]
  # roll = o["roll"]
  # yaw = o["yaw"]

  # test values
  pitch = 86.4
  roll = 2.3
  yaw = 22.9

  print("pitch {0} roll {1} yaw {2}".format(pitch, roll, yaw))

  # encoding floating point values as an ascii string...
  sendData = (str(pitch) + "_" + str(roll) + "_" + str(yaw)).encode('ascii')
  sock.sendto(sendData, (UDP_IP, UDP_PORT))

  time.sleep(UPDATE_INTERVAL)