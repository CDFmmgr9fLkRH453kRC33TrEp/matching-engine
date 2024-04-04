import pyaudio
import numpy as np
import matplotlib.pyplot as plt
import wave
import audioop

file = open('Audio_data', 'a+')

CHUNK = 256
FORMAT = pyaudio.paInt16
CHANNELS = 2
RATE = 20000
RECORD_SECONDS = 10
WAVE_OUTPUT_FILENAME = "output.wav"

p = pyaudio.PyAudio()
alldata = []
y_vals = []
x_vals = []
count = 1
stream = p.open(format=FORMAT, channels=CHANNELS, rate=RATE, input=True, frames_per_buffer=CHUNK)

for i in range(1, int(RATE / CHUNK * RECORD_SECONDS) + 1):
    data = stream.read(CHUNK)
    rms = (audioop.rms(data, 2))
    alldata.append(rms)
    if (i % 120 == 0):
        toAppend = round(np.mean(alldata),2)
        y_vals.append(toAppend)
        file.write(str(toAppend) + '\n')
        x_vals.append(count)
        count += 1
        alldata = []

stream.stop_stream()
stream.close()
p.terminate()

file.close()

plt.plot(x_vals, y_vals, label='Data Points', marker='o')
plt.xlabel('Time')
plt.ylabel('Avg Volume')
plt.show()


