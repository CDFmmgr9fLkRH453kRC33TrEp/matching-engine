from selenium import webdriver
from selenium.webdriver.common.by import By
import time
from PIL import Image
import matplotlib.pyplot as plt

file = open('TS_data', 'a+')

def calculate_average_pixel_value(image_path):
    image = Image.open(image_path)
    image = image.convert('L')
    pixel_data = list(image.getdata())
    average_pixel_value = sum(pixel_data) / len(pixel_data)
    return average_pixel_value

y_vals = []
x_vals = []
driver = webdriver.Edge()
driver.get("https://www.earthcam.com/usa/newyork/timessquare/?cam=tsrobo1")
livestream = driver.find_element(By.ID,value= 'videoPlayer_html5_api')

time.sleep(30)
for i in range(5):
    livestream.screenshot("screenshots/othertest.png")
    toAppend = calculate_average_pixel_value("screenshots/othertest.png")
    y_vals.append(toAppend)
    x_vals.append(i)
    file.write(str(toAppend) + '\n')
    time.sleep(5)

file.close()
plt.plot(x_vals, y_vals, label='Data Points', marker='o')
plt.xlabel('Time')
plt.ylabel('Avg Brightness')
plt.show()