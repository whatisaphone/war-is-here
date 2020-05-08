struct Vertex {
    float4 position : POSITION;
    float4 color : COLOR;
    float2 tex : TEXCOORD0;
};

struct Pixel {
    float4 position : SV_POSITION;
    float4 color : COLOR;
    float2 tex : TEXCOORD0;
};

Texture2D shaderTexture;
SamplerState SampleType;

Pixel vs_copy(Vertex input) {
    Pixel output = { input.position, input.color, input.tex };
    return output;
}

float4 ps_tex(Pixel input) : SV_TARGET {
    return shaderTexture.Sample(SampleType, input.tex);
}
