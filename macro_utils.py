import pyautogui
import time

def press_key(key, duration=0.1):
    """Press and release a key."""
    pyautogui.keyDown(key)
    time.sleep(duration)
    pyautogui.keyUp(key)

def move_mouse(x, y, duration=0.1):
    """Move the mouse to a position."""
    pyautogui.moveTo(x, y, duration)

def click_mouse(button='left', duration=0.1):
    """Click a mouse button."""
    pyautogui.mouseDown(button=button)
    time.sleep(duration)
    pyautogui.mouseUp(button=button)

def main(actions):
    """Execute a series of actions."""
    for action in actions:
        event = action.get('event')
        timestamp = action.get('timestamp')
        if 'Key pressed:' in event:
            key = event.split('Key pressed: ')[1]
            press_key(key, duration=0.1)
        elif 'Mouse moved to:' in event:
            coords = event.split('Mouse moved to: ')[1].strip('()')
            x, y = map(int, coords.split(', '))
            move_mouse(x, y, duration=0.1)
        elif 'Mouse clicked:' in event:
            button = event.split('Mouse clicked: ')[1]
            click_mouse(button=button, duration=0.1)
        time.sleep(timestamp / 1000.0)  # Convert milliseconds to seconds

# Example usage:
if __name__ == "__main__":
    example_actions = [
        {"event": "Key pressed: a", "timestamp": 100},
        {"event": "Mouse moved to: (100, 200)", "timestamp": 200},
        {"event": "Mouse clicked: left", "timestamp": 300}
    ]
    main(example_actions)
