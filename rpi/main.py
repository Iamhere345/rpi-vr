from sense_hat import SenseHat
import time
import socket
from random import randint

#! i couldn't get sense_emu working on windows. I might have another go on MacOS, but for now i may have to leave the proper testing
#! until i boot up the raspberry pi again. I've temporarily commented out SenseHAT-related code, and instead of emulator gryo data
#! i'm using constant test values

UPDATE_INTERVAL = 0.01

UDP_IP = "192.168.68.109"
UDP_PORT = 8080

# SenseHAT init
sense = SenseHat()
sense.clear()

# UDP init
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

while True:
  
  # it seems that with the way the raspberry pi is mounted
  # pitch and roll are swapped
  axis = sense.get_orientation()
  pitch = -axis["roll"]
  roll = axis["pitch"]
  yaw = axis["yaw"]

  # test values
  # pitch = randint(0, 180)
  # roll = randint(0, 180)
  # yaw = randint(0, 180)

  print("pitch {0} roll {1} yaw {2}".format(pitch, roll, yaw))

  # encoding floating point values as an ascii string...
  sendData = (str(pitch) + "_" + str(roll) + "_" + str(yaw)).encode('ascii')
  sock.sendto(sendData, (UDP_IP, UDP_PORT))

  time.sleep(UPDATE_INTERVAL)