"use client"
import Image from "next/image";
import { useState } from 'react';
import logo from "../public/vercel.svg";

export default function Home() {
  const [username, setUsername] = useState("")
  const [password, setPassword] = useState("")
  return (
    <main className="content ">
      <form className="signin">
        <label>Username</label>
        <br/>
        <input value={username} onChange={(e)=>setUsername(e.target.value)}/>
        <br/>
        <label>Password</label>
        <br/>
        <input value={password} onChange={(e)=>setPassword(e.target.value)}/>
        <br/>
        <button>Sign in</button>
      </form>
    </main>
  );
}
