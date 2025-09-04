function isString(s: unknown) {
  return typeof s === "string" || s instanceof String;
}

export { isString };
