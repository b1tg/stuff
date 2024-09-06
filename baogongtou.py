# 2024/09/02 code with claude
# desc: 监控当前活跃窗口，打印窗口信息，是浏览器的话打印 url
# refs：https://github.com/rize-io/get-windows/blob/main/Sources/windows/main.cc
import win32gui
import win32process
import psutil
import time
import comtypes.client
from ctypes import windll, create_unicode_buffer

# Initialize UI Automation
UIAutomation = comtypes.client.GetModule("UIAutomationCore.dll")
IUIAutomation = comtypes.client.CreateObject("{ff48dba4-60ef-4201-aa87-54103eef594e}", interface=UIAutomation.IUIAutomation)

def get_active_window_info():
    hwnd = win32gui.GetForegroundWindow()
    _, pid = win32process.GetWindowThreadProcessId(hwnd)
    
    try:
        process = psutil.Process(pid)
        process_name = process.name()
        process_path = process.exe()
    except psutil.NoSuchProcess:
        process_name = "Unknown"
        process_path = "Unknown"

    title = win32gui.GetWindowText(hwnd)
    
    rect = win32gui.GetWindowRect(hwnd)
    x, y, width, height = rect[0], rect[1], rect[2] - rect[0], rect[3] - rect[1]

    return {
        "hwnd": hwnd,
        "pid": pid,
        "name": process_name,
        "path": process_path,
        "title": title,
        "bounds": {
            "x": x,
            "y": y,
            "width": width,
            "height": height
        }
    }

def get_browser_url(hwnd):
    try:
        element = IUIAutomation.ElementFromHandle(hwnd)
        condition = IUIAutomation.CreatePropertyCondition(UIAutomation.UIA_ControlTypePropertyId, UIAutomation.UIA_EditControlTypeId)
        edit = element.FindFirst(UIAutomation.TreeScope_Descendants, condition)
        
        if edit:
            pattern = edit.GetCurrentPattern(UIAutomation.UIA_ValuePatternId)
            interface = pattern.QueryInterface(UIAutomation.IUIAutomationValuePattern)
            return interface.CurrentValue
    except:
        pass
    return "Unable to retrieve URL"

def is_browser(process_name):
    browsers = ["chrome.exe", "firefox.exe", "iexplore.exe", "msedge.exe", "opera.exe"]
    return process_name.lower() in browsers

def get_browser_mode(hwnd):
    try:
        element = IUIAutomation.ElementFromHandle(hwnd)
        condition = IUIAutomation.CreatePropertyCondition(UIAutomation.UIA_NamePropertyId, "InPrivate")
        inprivate = element.FindFirst(UIAutomation.TreeScope_Descendants, condition)
        
        if inprivate:
            return "incognito"
    except:
        pass
    return "normal"

def monitor_active_window():
    last_hwnd = None
    
    while True:
        window_info = get_active_window_info()
        hwnd = window_info['hwnd']
        
        if hwnd != last_hwnd:
            print(f"Active Window: {window_info['name']}")
            print(f"Title: {window_info['title']}")
            print(f"Process ID: {window_info['pid']}")
            print(f"Path: {window_info['path']}")
            print(f"Bounds: {window_info['bounds']}")
            
            if is_browser(window_info['name']):
                url = get_browser_url(hwnd)
                mode = get_browser_mode(hwnd)
                print(f"URL: {url}")
                print(f"Mode: {mode}")
            
            print("------------------------")
            
            last_hwnd = hwnd
        
        time.sleep(2)  # Check every second

if __name__ == "__main__":
    monitor_active_window()
