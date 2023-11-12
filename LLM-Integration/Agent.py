#OPENAI_API_KEY = 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
from langchain.agents import AgentExecutor, tool
from langchain.agents.format_scratchpad import format_to_openai_functions
from langchain.agents.output_parsers import OpenAIFunctionsAgentOutputParser
from langchain.chat_models import ChatOpenAI
from langchain.prompts import ChatPromptTemplate, MessagesPlaceholder
from langchain.tools.render import format_tool_to_openai_function
from langchain.memory import ConversationBufferMemory
from dotenv import load_dotenv
import json
import math

load_dotenv()

# Dummy data for testing
global_cube_positions = [{'id': 0, 'x': 330, 'y': 245, 'theta': 130},
                         {'id': 1, 'x': 331, 'y': 342, 'theta': 208}]

llm = ChatOpenAI(model_name="gpt-3.5-turbo", temperature=0)

def calculate_line_positions(start_x, start_y, end_x, end_y, num_robots):
    positions = []
    delta_x = (end_x - start_x) / (num_robots - 1)
    delta_y = (end_y - start_y) / (num_robots - 1)
    for i in range(num_robots):
        positions.append({
            'x': int(start_x + i * delta_x),
            'y': int(start_y + i * delta_y),
            'theta': 0 if delta_x >= 0 else 180
        })
    return positions

def calculate_grid_positions(top_left_x, top_left_y, num_robots, rows, columns, spacing):
    positions = []
    for i in range(rows):
        for j in range(columns):
            if len(positions) < num_robots:
                positions.append({
                    'x': top_left_x + j * spacing,
                    'y': top_left_y + i * spacing,
                    'theta': 0
                })
    return positions

def calculate_circle_positions(center_x, center_y, radius, num_robots):
    angle_step = (2 * math.pi) / num_robots
    return [
        {
            'x': int(center_x + radius * math.cos(angle * angle_step)),
            'y': int(center_y + radius * math.sin(angle * angle_step)),
            'theta': int(math.degrees(angle * angle_step)) % 360
        }
        for angle in range(num_robots)
    ]

@tool
def arrange_toios(shape: str, toio_positions: list) -> str:
    """
    Function that defines how to arrange Toio robots into a specified shape.
    Currently supports arranging into a circle.
    """
    if shape == 'circle':
        # Determine the center and radius based on current positions
        # This is a simplification. You might want to find the centroid and a suitable radius.
        center_x = sum(p['x'] for p in toio_positions) // len(toio_positions)
        center_y = sum(p['y'] for p in toio_positions) // len(toio_positions)
        radius = 100  # Example radius; you might want to calculate or parameterize this

        # Calculate the target positions for the Toio robots
        target_positions = calculate_circle_positions(center_x, center_y, radius, len(toio_positions))
        return json.dumps(target_positions)
    elif shape == 'line':
        # For a line, we use the first and last current positions as start and end points
        start_x = toio_positions[0]['x']
        start_y = toio_positions[0]['y']
        end_x = toio_positions[-1]['x']
        end_y = toio_positions[-1]['y']
        target_positions = calculate_line_positions(start_x, start_y, end_x, end_y, len(toio_positions))
        return json.dumps(target_positions)

    elif shape == 'grid':
        # For a grid, we assume top left as the starting point and calculate positions
        top_left_x = toio_positions[0]['x']
        top_left_y = toio_positions[0]['y']
        rows = 2  # Example, should be calculated or parameterized based on num_robots
        columns = math.ceil(len(toio_positions) / rows)
        spacing = 100  # Example spacing
        target_positions = calculate_grid_positions(top_left_x, top_left_y, len(toio_positions), rows, columns, spacing)
        return json.dumps(target_positions)

    return json.dumps([])

tools = [arrange_toios]

prompt = ChatPromptTemplate.from_messages([
    ("system", "You are an intelligent assistant capable of arranging Toio robots into shapes. You should return a list of final toio positions wrapped in square brackets [] "),
    ("user", "{input}"),
    MessagesPlaceholder(variable_name="agent_scratchpad"),
])


# Bind the tools to the LLM
llm_with_tools = llm.bind(
    functions=[format_tool_to_openai_function(t) for t in tools]
)

chat_history = []
output_parser = OpenAIFunctionsAgentOutputParser()
# Declare the agent
agent = {
    "input": lambda x: x["input"],
    "agent_scratchpad": lambda x: format_to_openai_functions(x.get('intermediate_steps', [])),
    "chat_history": lambda x: x["chat_history"]
} | prompt | llm_with_tools | output_parser

memory = ConversationBufferMemory(memory_key="chat_history", return_messages=True)
agent_executor = AgentExecutor(agent=agent, tools=tools, memory=memory, verbose=True)

# Function to invoke the LLM agent with user input and current Toio positions
def invoke_agent(user_input, toio_positions):
    # Format the toio positions into a JSON string
    toio_positions_str = json.dumps(toio_positions)
    
    # Prepare the input string for the LLM
    formatted_input = f"The user wants the Toios to form a {user_input}. The current positions are {toio_positions_str}. Output a final Toio positions in a list within square brackets []"
    
    # Invoke the agent with the formatted input string
    input_data = {
        "input": formatted_input,
        "chat_history": chat_history,
    }
    return agent_executor.invoke(input_data)

# Example usage
if __name__ == "__main__":
    user_input = "circle"  # Example user command
    result = invoke_agent(user_input, global_cube_positions)
    print(result)
