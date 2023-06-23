"use client";

import formStyles from "./form.module.scss";
import { addStudent } from "@/lib/api";
import { useState } from "react";

export default function AddStudentForm() {
  const [firstName, setFirstName] = useState("");
  const [lastName, setLastName] = useState("");
  const [gradeLvl, setGradeLvl] = useState("");
  const [status, setStatus] = useState("start"); // start, loading, success, err
  const [submitVal, setSumbitVal] = useState("Submit");

  async function handleSubmit(e) {
    e.preventDefault(); // prevent reload
    setStatus("loading");
    setSumbitVal("Loading...");

    const res = await addStudent(firstName, lastName, gradeLvl);
    if (res.success) {
      setStatus("success");
      setFirstName("");
      setLastName("");
      setGradeLvl("");
      setSumbitVal("Student Added!");
    } else {
      setStatus("err");
      setSumbitVal("Try Again");
    }
    console.log(res);
  }

  function handleFirstNameChange(e) {
    setSumbitVal("Submit");
    setFirstName(e.target.value);
  }

  function handleLastNameChange(e) {
    setSumbitVal("Submit");
    setLastName(e.target.value);
  }

  function handleGradeLvlChange(e) {
    setSumbitVal("Submit");
    setGradeLvl(e.target.value);
  }

  function handleSubmitVal() {
    if (status === "start") {
      return "Submit";
    } else if (status === "loading") {
      return "loading";
    } else if (status === "success") {
      return "success";
    } else if (status === "err") {
      return "err";
    }
  }

  return(
    <form className={formStyles.form} onSubmit={handleSubmit}>
      <div className={formStyles.textGroup}>
        <input type="text" placeholder="First name" value={firstName} onChange={handleFirstNameChange} disabled={status === "loading"} required/>
        <input type="text" placeholder="Last name" value={lastName} onChange={handleLastNameChange} disabled={status === "loading"} required/>
      </div>
      <div className={formStyles.radioGroup}>
        <label>Grade level</label>
        <div className={formStyles.btns}>
          <div>
            <input type="radio" id="9" name="gradeLvl" value="9" onChange={handleGradeLvlChange} checked={gradeLvl === "9"} disabled={status === "loading"} required/>
            <label htmlFor="9">9</label>
          </div>
          <div>
            <input type="radio" id="10" name="gradeLvl" value="10" onChange={handleGradeLvlChange} checked={gradeLvl === "10"} disabled={status === "loading"} />
            <label htmlFor="10">10</label>
          </div>
          <div>
            <input type="radio" id="11" name="gradeLvl" value="11" onChange={handleGradeLvlChange} checked={gradeLvl === "11"} disabled={status === "loading"} />
            <label htmlFor="11">11</label>
          </div>
          <div>
            <input type="radio" id="12" name="gradeLvl" value="12" onChange={handleGradeLvlChange} checked={gradeLvl === "12"} disabled={status === "loading"} />
            <label htmlFor="12">12</label>
          </div>
        </div>
      </div>
      <input type="submit" value={submitVal} disabled={!firstName || !lastName || !gradeLvl}/>
    </form>
  );
}
