class Headers extends Map {
  constructor(init) {
    super();
    if (init) {
      if (Array.isArray(init)) {
        for (let [name, value] of init) {
          this.append(name, value);
        }
      } else {
        for (let name in init) {
          this.append(name, init[name]);
        }
      }
    }
  }

  append(name, value) {
    let header = this.get(name);
    if (header) {
      this.set(name, header + ", " + value);
    } else {
      this.set(name, value);
    }
  }
}
