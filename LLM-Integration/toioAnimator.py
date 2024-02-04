from openai import OpenAI
import os
import json
from pythonosc import udp_client, dispatcher, osc_server
import threading
import re
import time

#Dummmy data
dummy_toio_positions = [{'id': 0, 'x': 0, 'y': 0, 'theta': 130},
                         {'id': 1, 'x': 1, 'y': 1, 'theta': 208},
                         {'id': 2, 'x': 1, 'y': 0, 'theta': 208},
                         {'id': 3, 'x': 0, 'y': 1, 'theta': 208}]

dummy_send = '[{"id": 0, "x": 125, "y": 288, "theta": 65}, {"id": 1, "x": 200, "y": 273, "theta": 61}, {"id": 2, "x": 210, "y": 205, "theta": 68}, {"id": 3, "x": 134, "y": 215, "theta": 59}]{"id": 0, "x": 125, "y": 288, "theta": 65}, {"id": 1, "x": 200, "y": 273, "theta": 61}, {"id": 2, "x": 210, "y": 205, "theta": 68}, {"id": 3, "x": 134, "y": 215, "theta": 59}]'

#Set up Assistant
gpt = OpenAI(
   api_key=os.environ.get("OPENAI_API_KEY"),
)

animation_thread = None #flag for if an animation is running in the background
delay = 0.5

#message_counter = 0

assistant = gpt.beta.assistants.create(
    name="2D Graphed Image Interpreter",
    instructions="You are a interpeter of a 2D graph. Given a list of points and connections between points\
    , you are meant to interpret what basic images or shapes they might represent. The 2D plane has these specifications:\
    the top left is (x=45,y=45), and the bottom right is (x=455,y=455).",
    tools=[{"type": "retrieval"}],
    model="gpt-4-1106-preview"
)

thread = gpt.beta.threads.create()

#Set up server for communicating with toios
client = udp_client.SimpleUDPClient("127.0.0.1", 4444)

global_toio_positions = []

def handle_toio_positions(unused_addr, *args):
    global global_toio_positions
    global_toio_positions.clear()
    global_toio_positions = [{'id': args[i], 'x': args[i+1], 'y': args[i+2], 'theta': args[i+3], 'group':args[i+4]} for i in range(0, len(args), 5)]


def handle_test_reply(unused_addr, message):
    """Handle the test_reply message from Processing."""
    print("Received from Processing: " + message)


def extract_json_from_output(output):
    # Regular expression to match a JSON list
    json_list_pattern = r'\[.*?\]'
    matches = re.findall(json_list_pattern, output)
    if matches:
        # Assuming the first match is the JSON list you need
        json_list = matches[0]
        return json.loads(json_list)  # Convert JSON string to Python list
    else:
        raise ValueError("No JSON list found in the output")

          
#handle individual call
def wait_for_completion(run_id):
    """
    Wait for completion of prompt to continue
    """
    i = 0
    while(True):
        run = gpt.beta.threads.runs.retrieve(
			thread_id=thread.id,
			run_id=run_id
		)
        if(run.status == "completed"):
            return
        time.sleep(2)
        i += 2
        print("Loading: " + str(i) +" s")
	

def complete_one_prompt(prompt):
    """
    Completes one prompt
    """
    gpt.beta.threads.messages.create(
        thread_id=thread.id,
        role="user",
        content=prompt,
    )
    run = gpt.beta.threads.runs.create(
        thread_id=thread.id,
        assistant_id=assistant.id,
    )
    wait_for_completion(run.id)
    messages = gpt.beta.threads.messages.list(
        thread_id=thread.id
    )
    return messages.data[0].content[0].text.value
	

def interpret_toios(user_input, toio_positions):
    """
    Interpret toios with help from user
    """
    toio_positions_str = json.dumps(toio_positions)
    print(toio_positions_str)
    extra_input = ''
    if(user_input != ''):
        extra_input = f"Here is some clarifying information: {user_input}."
    prompt = f"Here is my list of points: {toio_positions_str}. {extra_input} What shape or basic image could this \
        represent? Give your answer in one brief sentence."
    while(prompt != ''):
        mes = complete_one_prompt(prompt)
        print(mes)
        final_result = mes
        print('\n')
        prompt = input("Are any of these interpretations accurate? (Enter if correct, otherwise write further instruction): ")
    #Switch to new movements
    final_result = complete_one_prompt("Return the final interpretation as a one or two word answer")
    return final_result


def new_movements():
    """
    Generate new positions given interpretation
    """
    animation_prompt = input("What animation should the toios make?\n")
    num_frames = input("How many frames?\n")
    prompt = f"Based on your current understanding of the current positions, we want to generate a simple looping \
         animation based on the following prompt: {animation_prompt}. Rearrange the points {num_frames} times to form a simple \
        looping animation with {num_frames} frames. The response should given as {num_frames} frames, each within [], and delimited \
        by a semicolon ; (no spaces). Each frame should be in the same format as the positions were given to you. Only return the list of frames\
        with no extra words or description."
    if(animation_prompt != ''):
        new_locations = complete_one_prompt(prompt)
        frames = new_locations.split(";")
        start_animation(animate_movements, frames)


def start_animation(target, frames):
    """
    Start animation thread
    """
    global animation_thread
    if animation_thread is None:
        animation_thread = threading.Thread(target=target, args=[frames])
        animation_thread.start()
    else:
        animation_thread.join()
        animation_thread = threading.Thread(target=target, args=[frames])
        animation_thread.start()
        

def animate_movements(frames):
    """
    Run animations in the background
    """
    global delay
    i = 0
    while True:
        try:
            client.send_message("/new_positions", frames[i])
            i += 1
            if (i == len(frames)):
                i = 0
        except ValueError as e:
            print(str(e))
        time.sleep(delay)


def main():
    global animation_thread
    disp = dispatcher.Dispatcher()
    disp.map("/test_reply", handle_test_reply)
    disp.map("/cube_positions", handle_toio_positions)

    server = osc_server.ThreadingOSCUDPServer(("127.0.0.1", 4445), disp)

    # Use threading to start the OSC server without blocking
    server_thread = threading.Thread(target=server.serve_forever)
    server_thread.start()

    print("Listening for OSC messages on port 4445...")


    #while True:
    user_message = input("Add further information to interpret toio positions (optional): ")
    result = interpret_toios(user_message, global_toio_positions)
    print(result)
    while(True):
        update_locations = input("Would you like to invoke movement? (Enter if no): ")
        if(update_locations != ''):
            new_movements()
        else:
            if animation_thread:
                animation_thread.join()
            break
    
    server_thread.join()
    return

if __name__ == "__main__":
    main()
