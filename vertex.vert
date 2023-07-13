#version 400

layout (location = 0) in vec3 pos;

//out vec3 col;

void main(){
    gl_Position = vec4(pos, 1.0);
    //col = vec3(1.0, 0.0, 0.0);
}

