from launch import LaunchDescription
from launch_ros.actions import Node
from launch.substitutions import LaunchConfiguration

def generate_launch_description():
    return LaunchDescription([
        Node(
            package="demo_nodes_cpp",
            executable="talker",
            name="my_talker"
        ),
        LaunchConfiguration("param_value")
    ])