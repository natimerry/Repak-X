// VFX Updater - Color Application Logic

import type { ColorParam } from "../../types";

export function setNestedValue(
  obj: any,
  path: (string | number)[],
  value: Record<string, number>
): boolean {
  let current = obj;
  for (let i = 0; i < path.length - 1; i++) {
    if (current === undefined || current === null) {
      console.debug("[VFX] setNestedValue: path traversal failed at index", i, "path:", path);
      return false;
    }
    current = current[path[i]];
  }
  if (current === undefined || current === null) {
    console.debug("[VFX] setNestedValue: final parent is null/undefined, path:", path);
    return false;
  }
  const lastKey = path[path.length - 1];
  if (typeof current[lastKey] === "object" && typeof value === "object") {
    Object.assign(current[lastKey], value);
  } else {
    current[lastKey] = value;
  }
  console.debug("[VFX] setNestedValue: successfully set value at path", path);
  return true;
}

export function applyColorToJson(
  json: any,
  color: ColorParam
): boolean {
  console.debug("[VFX] Applying color to JSON", {
    paramName: color.paramName,
    path: color.path,
    rgba: color.rgba,
  });
  
  return setNestedValue(json, color.path, {
    R: color.rgba.R,
    G: color.rgba.G,
    B: color.rgba.B,
    A: color.rgba.A,
  });
}
