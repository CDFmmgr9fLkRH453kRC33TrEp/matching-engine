from selenium import webdriver
from selenium.webdriver.common.by import By
import time
from PIL import Image
import matplotlib.pyplot as plt
import math

file = open('JJs_data','a+')
driver = webdriver.Edge()
driver.get("https://dining.columbia.edu/content/jjs-place-0")
#y_vals = []
#x_vals = []
time.sleep(15)
data = []

for i in range(5):
    capacity = driver.find_element(By.CLASS_NAME, value= "indicator").text
    end = capacity.index('%')
    capacity = capacity[:end]
    avg = float(capacity)
    data.append(avg)
    runavg = sum(data) / len(data)
    file.write(str(((math.atan((avg - runavg)/3) /  (math.pi)) + 0.5)*10000) + '\n')
    #y_vals.append(float(capacity))
    #x_vals.append(i)
    driver.refresh()
    time.sleep(20)

file.close()

#plt.plot(x_vals, y_vals, label='Data Points', marker='o')
#plt.xlabel('Time')
#plt.ylabel('Percent Full')
#plt.show()