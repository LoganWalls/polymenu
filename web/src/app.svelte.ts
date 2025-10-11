class App {
  /**
   * Options passed to the program at startup 
   */
  options: Record<string, any>

  constructor(options: Record<string, any>) {
    this.options = options;
  }

  /**
   * Fetches the program input (either from STDIN or from a file, 
   * depending on how the program was called from the CLI).
   * @returns Promise that resolves to the json values that were passed to the program
   */
  input = async <T>(): Promise<T[]> => {
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
    return await response.json()
  }

  /**
   * Prints values to STDOUT, each on a separate line.
   * @param values The array of strings to print.
   * @returns Promise that resolves when printing is done.
   */
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

  /**
   * Runs a user-defined command and returns its output
   * @param name The name of the command to run
   * @param args The arguments to be passed to the command
   * @returns Promise that resolves to the (json) output of the command
   */
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
    return await response.json()
  }

  /**
   * Exits the program
   * @returns Promise that resolves when the program has exited
   * (in practice, this will not resolve because the process will exit)
   */
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
