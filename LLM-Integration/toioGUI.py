from openai import OpenAI
import os
import json
from pythonosc import udp_client, dispatcher, osc_server
import threading
import re
import time
import pygame

#Set up Assistant
gpt = OpenAI(
   api_key=os.environ.get("OPENAI_API_KEY"),
)

assistant = gpt.beta.assistants.create(
    name="2D Graphed Image Interpreter",
    instructions="You are a interpeter of a 2D graph. Given a list of grouped points, you are meant to interpret what basic images or shapes they might represent. The 2D plane has these specifications:\
    the top left corner of our plane is (x=45,y=45), and the bottom right corner is (x=455,y=455). You will also be asked to return altered sets of points, such that basic animations are created",
    tools=[{"type": "retrieval"}],
    model="gpt-4-0125-preview"
)

thread = gpt.beta.threads.create()

#Set up server for communicating with toios
client = udp_client.SimpleUDPClient("127.0.0.1", 4444)

global_toio_positions = []
user_input = ''
submitted = 0
playing = False
reset = False
animation_thread = None #flag for if an animation is running in the background
delay = 0.3
new_locations = None
gpt_thread = None
animation_ready = False
frames = None


def save_file(string_data, filename, folder_path = "animations"):
    os.makedirs(folder_path, exist_ok=True)
    file_path = os.path.join(folder_path, filename)
    with open(file_path, 'w', encoding='utf-8') as file:
        file.write(string_data)
    print(f"File saved successfully at {file_path}")


def handle_toio_positions(unused_addr, *args):
    global global_toio_positions
    global_toio_positions.clear()
    global_toio_positions = [{'id': args[i], 'x': args[i+1], 'y': args[i+2], 'theta': args[i+3], 'group':args[i+4]} for i in range(0, len(args), 5)]


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
        represent? Give your answer in one or two short sentences, no more."
    mes = complete_one_prompt(prompt)
    print(mes)
    final_result = complete_one_prompt("Return the final interpretation as a one or two word answer. Do not include a period at the end.")
    return final_result


def new_movements(animation_prompt):
    """
    Generate new positions given interpretation
    """
    global new_locations
    global animation_ready
    prompt = f"Based on your current understanding of the current shape, we want to generate a simple looping \
        animation based on the following prompt: {animation_prompt}. If relevant, recall the grouping of our points and what each of these groups might represent. Use this understanding to inform the each rearrangement of points. \
        Treat each frame as an adjustment of the original shape through moving each point. Not every point must move at each frame, only move the ones that make sense to move. Rearrange the points 4 times to form a simple looping \
        animation with 4 frames. The response should given as 5 frames, each within [], and delimited by a semicolon ; (no spaces). Each frame should be in the same format as the positions were given to you. Only return the list of frames\
        with no extra words or description."
    if(animation_prompt != ''):
        new_locations = complete_one_prompt(prompt)
        animation_ready = True


def start_animation(target):
    """
    Start animation thread
    """
    global animation_thread
    global playing
    playing = False
    print("d")
    if animation_thread is None:
        animation_thread = threading.Thread(target=target, args=[])
        animation_thread.start()
    else:
        #animation_thread.join()
        animation_thread = threading.Thread(target=target, args=[])
        animation_thread.start()
        

def animate_movements():
    """
    Run animations in the background
    """
    global delay
    global playing
    global reset
    global frames

    i = 0
    client.send_message("/new_positions", frames[0])
    while True:
        if playing:
            try:
                client.send_message("/new_positions", frames[i])
                i += 1
                if (i == len(frames)):
                    i = 0
            except ValueError as e:
                print(str(e))
        if reset:
            reset = False
            break
        time.sleep(delay)


def main():
    global animation_thread
    global new_locations
    global playing
    global reset
    global gpt_thread
    global animation_ready
    global frames
    running = True
    user_input = ''
    global submitted
    loading_animation = False
    example_playing = False
    disp = dispatcher.Dispatcher()
    disp.map("/cube_positions", handle_toio_positions)

    server = osc_server.ThreadingOSCUDPServer(("127.0.0.1", 4445), disp)

    # Use threading to start the OSC server without blocking
    server_thread = threading.Thread(target=server.serve_forever)
    server_thread.start()

    print("Listening for OSC messages on port 4445...")

    #Setting up GUI
    pygame.init()
    pygame.display.set_caption('Shape n Swarm')
    color_active = pygame.Color('lightskyblue3')
    current_step = "interpretation"
    logo = pygame.transform.scale(pygame.image.load \
            ('logo.png'), (200,100))
    display = pygame.display.set_mode([800,600])
    font = pygame.font.Font('freesansbold.ttf', 20)
    font2 = pygame.font.Font('freesansbold.ttf', 15)
    font3 = pygame.font.Font('freesansbold.ttf', 30)
    font4 = pygame.font.Font('freesansbold.ttf', 40)
    result = "None"
    clock = pygame.time.Clock()
    input_rect = pygame.Rect(50,320,140,35)
    box_active = False
    buttonText = font3.render("Send Instructions", True, (0,0,0), (60,179,113))
    buttonRect = buttonText.get_rect()
    buttonRect.center = (400,410)
    saveText = font.render("Save", True, (0,0,0), (60,179,113))
    saveRect = saveText.get_rect()
    saveRect.center = (300,450)
    resetText = font.render("Reset", True, (0,0,0), (60,179,113))
    resetRect = resetText.get_rect()
    resetRect.center = (400,450)
    e1Text = font.render("Example 1: Wave", True, (0,0,0), (60,179,113))
    e1Rect = e1Text.get_rect()
    e1Rect.center = (150,550)
    e2Text = font.render("Example 2: Jumping Guy", True, (0,0,0), (60,179,113))
    e2Rect = e2Text.get_rect()
    e2Rect.center = (400,550)
    e3Text = font.render("Example 3: Not ready", True, (0,0,0), (60,179,113))
    e3Rect = e3Text.get_rect()
    e3Rect.center = (650,550)
    #Main loop
    while running:
        if playing:
            playText = font.render("Pause", True, (0,0,0), (60,179,113))
        else:
            playText = font.render("Play", True, (0,0,0), (60,179,113))
        playRect = playText.get_rect()
        playRect.center = (500,450)
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
            if event.type == pygame.MOUSEBUTTONDOWN:
                if input_rect.collidepoint(event.pos):
                    box_active = True
                elif buttonRect.collidepoint(event.pos):
                    if (current_step == "interpretation"):
                        result = interpret_toios(user_input, global_toio_positions)
                        current_step = "animation"
                        user_input = ''
                        filename = result.split(" ")[0]
                        filename = filename + ".txt"
                        print(filename)
                    else:
                        if gpt_thread is None:
                            gpt_thread = threading.Thread(target=new_movements, args=[user_input])
                            gpt_thread.start()
                        else:
                            gpt_thread.join()
                            gpt_thread = threading.Thread(target=new_movements, args=[user_input])
                            gpt_thread.start()
                        loading_animation = True
                        user_input = ''
                elif saveRect.collidepoint(event.pos):
                    if new_locations is not None and filename is not None:
                        save_file(new_locations, filename)
                elif playRect.collidepoint(event.pos):
                    if playing:
                        playing = False
                    else:
                        playing = True
                elif resetRect.collidepoint(event.pos):
                    reset = True
                    playing = False
                    animation_thread = None
                    current_step = "interpretation"
                    result = None
                    user_input = ''
                    filename = None
                elif e1Rect.collidepoint(event.pos):
                    with open("animations/wave.txt", 'r') as file:
                        # Read the first line from the file
                        tmp = file.read()
                    if tmp[1] == "[":
                        tmp = tmp[1:-1]
                    frames = tmp.split(";")
                    example_playing = True
                    playing = False
                    start_animation(animate_movements)
                elif e2Rect.collidepoint(event.pos):
                    with open("animations/jumping.txt", 'r') as file:
                        tmp = file.read()
                    if tmp[1] == "[":
                        tmp = tmp[1:-1]
                    frames = tmp.split(";")
                    example_playing = True
                    playing = False
                    start_animation(animate_movements)
                elif e3Rect.collidepoint(event.pos):
                    with open("animations/jumping.txt", 'r') as file:
                        tmp = file.read()
                    if tmp[1] == "[":
                        tmp = tmp[1:-1]
                    frames = tmp.split(";")
                    example_playing = True
                    playing = False
                    start_animation(animate_movements)
                elif readyRect.collidepoint(event.pos):
                    if animation_ready:
                        print("a")
                        loading_animation = False
                        if new_locations[1] == "[":
                            tmp = new_locations[1:-1]
                        else:
                            tmp = new_locations
                        print(tmp)
                        frames = tmp.split(";")
                        print("c")
                        playing = False
                        example_playing = False
                        animation_ready = False
                        start_animation(animate_movements)
                else:
                    box_active = False
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_BACKSPACE:
                    user_input = user_input[:-1]
                else:
                    user_input += event.unicode
        display.fill((255,255,255))
        if box_active:
            color = color_active
        else:
            color = (200,200,200)
        display.fill((255,255,255))
        pygame.draw.rect(display, color, input_rect)
        text_surface = font2.render(user_input, True, (0,0,0))
        display.blit(text_surface, (input_rect.x+5, input_rect.y+5))
        display.blit(logo, (600,0))
        input_rect.w = max(700, text_surface.get_width()+10)
        instructionsText = font.render("Please input instructions for the {} of your design".format(current_step), True, (0,0,0), (255,255,255))
        instructionsRect = instructionsText.get_rect()
        instructionsRect.center = (400,300)
        outText = font.render("Current interpretation of your design:", True, (0,0,0), (255,255,255))
        outRect = outText.get_rect()
        outRect.center = (400,220)
        wordText = font3.render(result, True, (0,0,0), (255,255,255))
        wordRect = wordText.get_rect()
        wordRect.center = (400,260)
        titleText = font4.render("Shape n' Swarm", True, (0,0,0), (255,255,255))
        titleRect = titleText.get_rect()
        titleRect.center = (400, 100)
        descriptionText = font.render("Smart Swarm Robots for Creative Interaction and Play Using LLMs", True, (0,0,0), (255,255,255))
        descriptionRect = descriptionText.get_rect()
        descriptionRect.center = (400, 150)
        if animation_ready:
            readyText = font.render("Animation ready! (press to play)", True, (0,0,0), (60,179,113))
        else:
            readyText = font.render("Animation loading...", True, (0,0,0), (255,255,255))
        readyRect = readyText.get_rect()
        readyRect.center = (400, 500)
        display.blit(descriptionText, descriptionRect)
        display.blit(titleText, titleRect)
        display.blit(buttonText, buttonRect)
        display.blit(outText, outRect)
        display.blit(wordText, wordRect)
        display.blit(instructionsText, instructionsRect)
        display.blit(e1Text, e1Rect)
        display.blit(e2Text, e2Rect)
        display.blit(e3Text, e3Rect)
        if loading_animation:
            display.blit(readyText, readyRect)
        if animation_thread is not None:
            if example_playing is False:
                display.blit(saveText, saveRect)
                display.blit(resetText, resetRect)
            display.blit(playText, playRect)
        pygame.display.update()
        clock.tick(60)  
    server_thread.join()
    pygame.display.quit()
    pygame.quit()
    return

if __name__ == "__main__":
    main()

    
