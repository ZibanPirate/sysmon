import { EventCallback, listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

let err: Error | null = null;

console.log(JSON.stringify("event", null, 2));

let callback: EventCallback<unknown> = (event) => {
  // alert(JSON.stringify(event, null, 2));
  console.log(event);
  // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
  // event.payload is the payload object
};

try {
  const unlisten = await listen("network-info", (...args) => callback(...args));
} catch (error) {
  err = error as Error;
  console.error(error);
}

function App() {
  const [greetMsg, setGreetMsg] = useState("hhh");
  const [name, setName] = useState("");

  useEffect(() => {
    callback = (event) => {
      setGreetMsg(JSON.stringify(event, null, 2));
    };

    return () => {
      callback = () => {};
    };
  }, []);

  return (
    <div
      className="container"
      onClick={() => {
        setGreetMsg(JSON.stringify(err, null, 2));
      }}
    >
      {greetMsg}
    </div>
  );
}

export default App;
