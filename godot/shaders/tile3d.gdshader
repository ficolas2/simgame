shader_type spatial;
render_mode unshaded, cull_disabled, depth_draw_always;

uniform sampler2D tex : source_color, filter_nearest, repeat_disable;
uniform float alpha_scissor_threshold : hint_range(0.0, 1.0) = 0.5;

varying vec4 v_cd;

void vertex() {    
	v_cd = INSTANCE_CUSTOM;
}

void fragment() {
	ivec2 resI = textureSize(tex, 0);
	vec2 res = vec2(resI);
	vec2 inv = 1.0 / res;

	vec2 tileSizeUV = vec2(1./11., 1./11.);
	vec2 tileOriginUV = v_cd.xy + inv * 0.5;
	
	vec2 atlas_uv = UV * tileSizeUV + tileOriginUV;
	vec4 col = texture(tex, atlas_uv);

    if (col.a < alpha_scissor_threshold) {
        discard;
    }

    ALBEDO = col.rgb;
}
