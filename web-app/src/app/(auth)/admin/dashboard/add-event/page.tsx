"use client";

import formStyles from "../form.module.scss";
import { addEvent } from "@/lib/api";
import { useState } from "react";

function AddEventForm() {
  const [name, setName] = useState("");
  const [points, setPoints] = useState("");
  const [status, setStatus] = useState("start"); // start, loading, success, err
  const [submitVal, setSumbitVal] = useState("Submit");

  async function handleSubmit(e) {
    e.preventDefault(); // prevent reload
    setStatus("loading");
    setSumbitVal("Loading...");

    const res = await addEvent(name, points);
    if (res.success) {
      setStatus("success");
      setName("");
      setPoints("");
      setSumbitVal("Event Added!");
    } else {
      setStatus("err");
      setSumbitVal("Try Again");
    }
    console.log(res);
  }

  function handleNameChange(e) {
    setSumbitVal("Submit");
    setName(e.target.value);
  }

  function handlePointsChange(e) {
    setSumbitVal("Submit");
    setPoints(e.target.value);
  }

  return(
    <form className={formStyles.form} onSubmit={handleSubmit}>
      <div className={formStyles.textGroup}>
        <input type="text" placeholder="Name" value={name} onChange={handleNameChange} disabled={status === "loading"} required/>
        <input type="number" placeholder="Points" value={points} onChange={handlePointsChange} disabled={status === "loading"} required/>
      </div>
      <input type="submit" value={submitVal} disabled={!name || !points}/>
    </form>
  );
}

export default function AdminAddEventPage() {
  return (
    <main>
      <AddEventForm />
    </main>
  );
}