class App {
  options: Record<string, any>

  constructor(options: Record<string, any>) {
    this.options = options;
  }

  input = async () => {
    const request = new Request("input", {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
    });
    const response = await window.fetch(request);
    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }
    return response.json()
  }

  print = async (values: string[]) => {
    const request = new Request("print", {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ values }),
    });
    const response = await window.fetch(request);
    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }
  }

  runCommand = async (name: string, args: Record<string, string>) => {
    const request = new Request(`command/${name}`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({ args }),
    });
    const response = await window.fetch(request);
    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }
    return response.json()
  }

  close = async () => {
    const request = new Request("close", {
      method: "PUT",
    });
    const response = await window.fetch(request);
    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }
  }
}

const request = new Request("options", {
  method: "GET",
  headers: {
    "Content-Type": "application/json",
  },
});
const response = await window.fetch(request);
if (!response.ok) {
  throw new Error(`HTTP error! Status: ${response.status}`);
}
const options = await response.json();

export const app = new App(options);
