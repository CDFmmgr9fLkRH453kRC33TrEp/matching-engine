from selenium import webdriver
from selenium.webdriver.common.by import By
import time
from PIL import Image
import matplotlib.pyplot as plt

file = open('JJs_data','a+')
driver = webdriver.Edge()
driver.get("https://dining.columbia.edu/content/jjs-place-0")
y_vals = []
x_vals = []
time.sleep(15)

for i in range(5):
    capacity = driver.find_element(By.CLASS_NAME, value= "indicator").text
    end = capacity.index('%')
    capacity = capacity[:end]
    file.write(capacity + '\n')
    y_vals.append(float(capacity))
    x_vals.append(i)
    driver.refresh()
    time.sleep(15)

file.close()

plt.plot(x_vals, y_vals, label='Data Points', marker='o')
plt.xlabel('Time')
plt.ylabel('Percent Full')
plt.show()