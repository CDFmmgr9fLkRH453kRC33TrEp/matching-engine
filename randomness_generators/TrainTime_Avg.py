from selenium import webdriver
from selenium.webdriver.common.by import By
import time
from PIL import Image
import matplotlib.pyplot as plt
#doesn't account for time = "now", but that ends up adding more variance

file = open('Train_data','a+')
driver = webdriver.Edge()
driver.get("https://realtimerail.nyc/stops/117")
time.sleep(10)
y_vals = []
x_vals = []

for i in range(5):
    times = driver.find_elements(By.CLASS_NAME, value= "time")
    avg = sum(int("".join([s for s in t.text if s.isdigit()])) for t in times)
    y_vals.append(avg)
    x_vals.append(i)
    file.write(str(avg) + '\n')
    driver.refresh()
    time.sleep(10)

file.close()
plt.plot(x_vals, y_vals, label='Data Points', marker='o')
plt.xlabel('Time')
plt.ylabel('Avg Arrival')
plt.show()