from sense_emu import SenseHat
import time
import socket
from random import randint

#! In this state the script only works when testing on a raspberry pi
#! to test this on a local device, change the ip address to 127.0.0.1 (and do the same in main.rs),
#! comment out all the SenseHat stuff, and uncomment the test values

UPDATE_INTERVAL = 0.5

UDP_IP = "192.168.68.118"
UDP_PORT = 8080

# SenseHAT init
sense = SenseHat()
sense.clear()

# UDP init
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

while True:
  
  axis = sense.get_orientation()
  pitch = axis["pitch"]
  roll = axis["roll"]
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
