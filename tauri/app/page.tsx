"use client";
// import Image from "next/image";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
// import logo from "../public/vercel.svg";

export default function Home() {
  const [greeting, setGreeting] = useState("");
  const [grades, setGrades] = useState("");
  const [subjects, setSubjects] = useState("");
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  useEffect(() => {
    invoke<string>("greet", { name: "Next.js" })
      .then((result) => setGreeting(result))
      .catch(console.error);
  }, []);

  function signin(e: any) {
    e.preventDefault();
    console.log(`sign in ${username} ${password}`);
    invoke<string>("subjects", { username, password })
      .then((result) => setSubjects(result))
      .catch(console.error);
    console.log(subjects);
  }
  function chooseSubject(d: string) {
    invoke<string>("grades", { subject: d })
      .then((result) => setGrades(result))
      .catch(console.error);
    console.log(subjects);
  }
  return (
    <main className="content ">
      <h1>{grades}</h1>
      <form
        className={grades || subjects ? "none" : "signin"}
        onSubmit={(e) => signin(e)}
      >
        <label>Username</label>
        <br />
        <input value={username} onChange={(e) => setUsername(e.target.value)} />
        <br />
        <label>Password</label>
        <br />
        <input value={password} onChange={(e) => setPassword(e.target.value)} />

        <br />
        <button className="signButton">Sign in</button>
      </form>
      {/*<h1 className="header">Choose a subject: </h1>*/}
      <div className={subjects ? "subjects" : "none"}>
        {subjects.split(",").map((d) => {
          return (
            <button
              key={d}
              onClick={() => chooseSubject(d)}
              className="subject"
            >
              {d}
            </button>
          );
        })}
      </div>
    </main>
  );
}
