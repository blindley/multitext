This project contains a parser for a format I call Multitext.

Multitext is a simple text file format that allows you to store multiple text files in one. Its original purpose was to allow all of the shader stages for an OpenGL shader program to be stored in a single file, for easily switching between them and making sure they match up. Of course, the format can be used to store any kind of text file. This file is a valid multitext file holding a vertex shader and a fragment shader, while also describing the format.

@@@ multitext header
The parser looks for the first line that contains the string 'multitext header', as above. The initial characters on that same line are used as a marker for indicating the start of each contained file. This allows you to choose any sequence of characters that won't conflict with the format of the files you are trying to store. The marker is trimmed for whitespace on the right, but not the left. Note that the marker can appear in your files, as long as it is not at the start a line. The marker I've chosen for this file is '@@@', but you can choose any sequence of characters apart from the literal string 'multitext header'.

Once the parser has identified the marker, it simply separates the file into a set of strings, demarcated by lines starting with the marker. The remaining text on the line that started with a marker is used (after being trimmed of whitespace on the left and right) as a key in a hash map that matches that key to the text that follows it until the next marker. The 'multiline header' key is included in this hash map.

If this file is parsed, will produce a hash map with 3 entries. The first entry will have the key 'multiline header', and will be matched to this text describing the format. The second entry will have the key 'vertex shader', and will contain the glsl vertex shader code provided below. The final entry will have the key 'fragment shader', and contain the final bit of shader code. Any text above the first marker line will be discarded.

@@@ vertex shader
#version 430 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;
out vec3 v_color;
void main() {
    gl_Position = vec4(v_position, 1.0);
    v_color = color;
}

@@@ fragment shader
#version 430 core
in vec3 v_color;
out vec4 f_color;
void main() {
    f_color = vec4(v_color, 1.0);
}
