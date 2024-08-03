"use client";
// import Image from "next/image";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
// import logo from "../public/vercel.svg";

export default function Home() {
  const [grades, setGrades] = useState("");
  const [subjects, setSubjects] = useState("");
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  function signin(e: any) {
    e.preventDefault();
    console.log(`sign in ${username} ${password}`);
    invoke<string>("subjects", { username, password })
      .then((result) => {
        console.log(result);
        setSubjects(result[0]);
        setGrades(result[1]);
        console.log(result[1]);
      })
      .catch(console.error);
  }
  function chooseSubject(chose: string) {
    let parsed = JSON.parse(`[${grades.slice(0, -1)}]`);
    let znamky: Array<number> = [];
    let prumery: Array<number> = [];
    parsed.forEach((p: Array<string>) => {
      console.log(p);
      if (chose == p[0]) {
        let newZnamka = parseInt(p[1]) * parseInt(p[2]);
        znamky.push(newZnamka);
        prumery.push(parseInt(p[2]));
      }
    });
    console.log(znamky);
    let prumer =
      znamky.reduce((partialSum, a) => partialSum + a, 0) /
      prumery.reduce((partialSum, a) => partialSum + a, 0);
    console.log(prumer);
  }
  console.log(subjects.split(","));
  return (
    <main className="content">
      <h1>{subjects}</h1>
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
      <div className={subjects ? "subjects" : "none"}>
        <h1 className="header">Choose a subject: </h1>
        {subjects
          .slice(0, -1)
          .split(",")
          .map((d: string) => {
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
