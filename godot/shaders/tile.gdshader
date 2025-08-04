shader_type canvas_item;

varying vec4    v_cd;

void vertex() {
    v_cd = INSTANCE_CUSTOM;
}

void fragment() {// compute 1px in UV‐space:
	ivec2 resI = textureSize(TEXTURE, 0);
	vec2 res = vec2(resI);
	vec2 inv = 1.0 / res;

	vec2 tileSizeUV = vec2(1./11., 1./11.);
	vec2 tileOriginUV = v_cd.xy + inv * 0.5;

	vec2 atlas_uv = UV * tileSizeUV + tileOriginUV;

	vec4 col = texture(TEXTURE, atlas_uv);
	float lum = dot(col.xyz, vec3(0.299, 0.587, 0.114));
	vec3 gray = vec3(lum);
	float t = v_cd.z;

	vec3 desat  = mix(col.rgb, gray, t * .7);
	vec3 dark   = mix(desat, vec3(0.0), t * .3);
	COLOR = vec4(dark, col.a);
}