export type JsonPrimitive = string | number | boolean | null;
export type JsonValue = JsonPrimitive | JsonObject | JsonArray;
export interface JsonObject { [key: string]: JsonValue }
export interface JsonArray extends Array<JsonValue> { }

export const apiRoutePrefix = "api"

export class App {
  /**
   * Options passed to the program at startup 
   */
  options: Record<string, JsonValue>

  constructor(options: Record<string, JsonValue>) {
    this.options = options;
  }

  /**
   * Creates a new `App` instance by fetching user-provided options from the server.
   * @returns a new `App` instance
   */
  static async fromFetchedOptions(): Promise<App> {
    await App.establishSession();
    const optionsRequest = new Request(`${apiRoutePrefix}/options`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
    });
    const response = await window.fetch(optionsRequest);
    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }
    const options: Record<string, JsonValue> = await response.json();
    return new App(options);
  }


  /**
   * Use the auth token injected by polymenu's rust binary
   * to establish a session. This sets a session cookie that
   * allows your app to make requests to the polymenu server.
   */
  static async establishSession() {
    const sessionRequest = new Request("/session", {
      method: "POST",
      headers: {
        "Authorization": `Bearer ${(window as any).__AUTH_TOKEN__}`
      },
    });
    const response = await window.fetch(sessionRequest);
    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }
  }

  /**
   * Fetches the program input (either from STDIN or from a file, 
   * depending on how the program was called from the CLI).
   * @returns Promise that resolves to the json values that were passed to the program
   */
  input = async <T>(): Promise<T[]> => {
    const request = new Request(`${apiRoutePrefix}/input`, {
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
   * Prints `values` to STDOUT, each on a separate line.
   * @param values An array of (JSON-serializable) values to print.
   * @returns Promise that resolves when printing is done.
   */
  print = async (values: JsonValue | JsonValue[]) => {
    // Print endpoint always expects an array at the root
    if (!Array.isArray(values)) {
      values = [values]
    }

    const request = new Request(`${apiRoutePrefix}/print`, {
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
  runCommand = async (name: string, args: Record<string, string> = {}) => {
    const request = new Request(`${apiRoutePrefix}/command/${name}`, {
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
    const request = new Request(`${apiRoutePrefix}/close`, {
      method: "PUT",
    });
    const response = await window.fetch(request);
    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }
  }
}
