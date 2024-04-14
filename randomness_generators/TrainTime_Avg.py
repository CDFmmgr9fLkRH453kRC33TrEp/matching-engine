from selenium import webdriver
from selenium.webdriver.common.by import By
import time
from PIL import Image
import matplotlib.pyplot as plt
import math
#doesn't account for time = "now", but that ends up adding more variance

file = open('Train_data','a+')
driver = webdriver.Edge()
driver.get("https://realtimerail.nyc/stops/117")
time.sleep(10)
y_vals = []
#x_vals = []
#if we actually want avg wait time, should use the difference between each time
tot = 0
count = 0

for i in range(60):
    myl = []
    for n in range(5):
        times = driver.find_elements(By.CLASS_NAME, value= "time")
        if (n < len(times)):
            myl.append(int("0" + "".join([s for s in times[n].text if s.isdigit()])))
        else:
            myl.append(0)
    avg = sum(myl)   
    ##avg = sum(int("0" + "".join([s for s in t.text if s.isdigit()])) for t in times)
    count += 1
    tot += avg
    #y_vals.append(avg)
    #x_vals.append(i)
    runavg = tot / count
    file.write(str(((math.atan((avg - runavg) / 6) /  (math.pi)) + 0.5)*10000) + '\n')
    driver.refresh()
    time.sleep(10)

file.close()
#plt.plot(x_vals, y_vals, label='Data Points', marker='o')
#plt.xlabel('Time')
#plt.ylabel('Avg Arrival')
#plt.show()
