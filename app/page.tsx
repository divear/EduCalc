"use client";
// import Image from "next/image";
import { invoke } from "@tauri-apps/api/tauri";
// import logo from "../public/vercel.svg";
import { useState } from "react";

export default function Home() {
  const [error, setError] = useState("");
  const [grades, setGrades] = useState("");
  const [subjects, setSubjects] = useState("");
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [newGrade, setNewGrade] = useState("");
  const [newValue, setNewValue] = useState("1");
  const [newAverage, setNewAverage] = useState<number>();
  const [currAverage, setCurrAverage] = useState<number>();
  const [chosenGrades, setChosenGrades] = useState<number[]>();
  const [chosenShowGrades, setChosenShowGrades] = useState<number[]>();
  const [chosenValues, setChosenValues] = useState<number[]>();
  const [showPass, setShowPass] = useState(false);

  function signin(e: any) {
    e.preventDefault();
    if (!username || !password) {
      setError("You have to fill all fields!");
      return;
    }
    invoke<string>("subjects", { username, password })
      .then((result) => {
        console.log(result);
        if (result[0] == "could not get subjects") {
          setError("Could not get subjects.");
          return;
        }
        setSubjects(result[0]);
        setGrades(result[1]);
        console.log(result[1]);
      })
      .catch(console.error);
  }
  function chooseSubject(chose: string) {
    let parsed = JSON.parse(`[${grades.slice(0, -1)}]`);
    let znamky: Array<number> = [];
    let showZnamky: Array<number> = [];
    let vahy: Array<number> = [];
    console.log(parsed);

    parsed.forEach((p: Array<string>) => {
      if (chose == p[0]) {
        console.log(p);
        let newZnamka = parseFloat(p[1]) * parseFloat(p[2]);
        znamky.push(newZnamka);
        showZnamky.push(parseFloat(p[1]));
        vahy.push(parseFloat(p[2]));
        console.log(vahy);
      }
    });
    console.log(vahy);
    setChosenGrades(znamky);
    setChosenValues(vahy);
    setChosenShowGrades(showZnamky);
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
    if (!grade || !value) {
      setNewAverage(currAverage);
      return;
    }
    console.log(grade);
    console.log(value);

    let gradTemp = chosenGrades.toSorted(); // to copy by value
    let valuTemp = chosenValues.toSorted(); // to copy by value
    gradTemp.push(parseFloat(grade) * parseFloat(value));
    valuTemp.push(parseFloat(value));

    let prumer =
      gradTemp.reduce((partialSum, a) => partialSum + a, 0) /
      valuTemp.reduce((partialSum, a) => partialSum + a, 0);

    setNewAverage(prumer);
  }

  return (
    <main className="content">
      <div className={error ? "error" : "none"}>
        <h1>{error}</h1>
      </div>
      <form
        className={grades || subjects ? "none" : "signin"}
        onSubmit={(e) => {
          signin(e);
        }}
      >
        <h1>Username</h1>
        <br />
        <input value={username} onChange={(e) => setUsername(e.target.value)} />
        <br />
        <h1>Password</h1>
        <br />
        <div className="pass">
          <input
            type={showPass ? "text" : "password"}
            value={password}
            onChange={(e) => setPassword(e.target.value)}
          />
          <button
            id="showPass"
            type="button"
            onClick={() => setShowPass(!showPass)}
          >
            {showPass ? "👁" : "🔒"}
          </button>
        </div>
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
        <button
          className="backButton"
          onClick={() => {
            setChosenGrades(undefined);
            setNewAverage(undefined);
            setNewGrade("");
            setNewValue("");
          }}
        >
          🔙
        </button>
        <p>
          Your current average:{" "}
          <span className="currentAverage">{currAverage}</span>
        </p>
        <p>
          Your grades:{" "}
          <span>
            {chosenShowGrades?.toString().replace("", ", ").substring(1)}
          </span>
        </p>
        <p>
          Their weights:{" "}
          <span>{chosenValues?.toString().replace("", ", ").substring(1)}</span>
        </p>
        <p>New grade:</p>
        <input
          autoFocus={true}
          value={newGrade}
          onChange={(e) => {
            setNewGrade(e.target.value);
            calcAverage(e.target.value, newValue);
          }}
        />
        <p>The grade&apos;s weight:</p>
        <input
          value={newValue}
          onChange={(e) => {
            setNewValue(e.target.value);
            calcAverage(newGrade, e.target.value);
          }}
        />
        <h1 className="newAverage">
          <p className={newAverage ? "" : "none"}>Your new average:</p>
          {newAverage}
        </h1>
      </div>
    </main>
  );
}
