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
  const [newGrade, setNewGrade] = useState("");
  const [newValue, setNewValue] = useState("1");
  const [newAverage, setNewAverage] = useState<number>();
  const [currAverage, setCurrAverage] = useState<number>();
  const [chosenGrades, setChosenGrades] = useState<number[]>();
  const [chosenValues, setChosenValues] = useState<number[]>();

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
    let vahy: Array<number> = [];
    parsed.forEach((p: Array<string>) => {
      if (chose == p[0]) {
        console.log(p);
        let newZnamka = parseInt(p[1]) * parseInt(p[2]);
        znamky.push(newZnamka);
        vahy.push(parseInt(p[2]));
        console.log(vahy);
      }
    });
    console.log(vahy);
    setChosenGrades(znamky);
    setChosenValues(vahy);
    let prumer =
      znamky.reduce((partialSum, a) => partialSum + a, 0) /
      vahy.reduce((partialSum, a) => partialSum + a, 0);
    console.log(prumer);
    setCurrAverage(prumer);
  }

  function calcAverage(grade: string, value: string) {
    console.log(chosenGrades);
    console.log(chosenValues);
    if (!chosenValues || !chosenGrades) return;
    let gradTemp = chosenGrades.toSorted(); // to copy by value
    let valuTemp = chosenValues.toSorted(); // to copy by value
    gradTemp.push(parseInt(grade) * parseInt(value));
    valuTemp.push(parseInt(value));
    console.log(gradTemp);
    console.log(valuTemp);

    let prumer =
      gradTemp.reduce((partialSum, a) => partialSum + a, 0) /
      valuTemp.reduce((partialSum, a) => partialSum + a, 0);
    console.log(prumer);

    setNewAverage(prumer);
  }

  return (
    <main className="content">
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
        <p className="text-sm m-5">
          Your credentials are used only to sign into EduPage and aren&apos;t
          saved anywhere
        </p>
      </form>
      <div className={subjects && !chosenGrades ? "subjects" : "none"}>
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
      <div className={chosenGrades ? "calculator" : "none"}>
        <p>
          Your current average: <span>{currAverage}</span>
        </p>
        <p>
          Your grades:{" "}
          <span>{chosenGrades?.toString().replace("", ",").substring(1)}</span>
        </p>
        <p>New grade:</p>
        <input
          value={newGrade}
          onChange={(e) => {
            setNewGrade(e.target.value);
            calcAverage(e.target.value, newValue);
          }}
        />
        <p>The grade&apos;s value:</p>
        <input
          value={newValue}
          onChange={(e) => {
            setNewValue(e.target.value);
            calcAverage(newGrade, e.target.value);
          }}
        />
        <h1>{newAverage}</h1>
      </div>
    </main>
  );
}
