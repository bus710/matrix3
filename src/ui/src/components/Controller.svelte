<script>
    import { get } from "svelte/store";
    import { stored_dots } from "../stores";
    import { stored_color } from "../stores";
    import { stored_select } from "../stores";

    let color = get(stored_color);
    let sliderR = color.sliderR;
    let sliderG = color.sliderG;
    let sliderB = color.sliderB;
    let alpha = color.alpha;

    // Change the stored color value to change selected buttons in Matrix component
    function slider_changed(id, value) {
        console.log(id, ":", value);
        stored_color.set({ sliderR, sliderG, sliderB, alpha });
    }

    // Select all buttons in Matrix component
    function select_all() {
        console.log("all");
        stored_color.set({ sliderR: sliderR, sliderG: sliderG, sliderB: sliderB, alpha: 255 });
        stored_select.set("all");
        stored_select.set("");
    }

    // Select none of the buttons in Matrix component
    function select_none() {
        console.log("none");
        stored_select.update((v) => (v = "none"));
        stored_select.set("");
    }

    // Post to the random API of backend
    async function random() {
        console.log("random");
        let host = "http://" + location.hostname + ":8000/v1/random";
        console.log(host);
        let options = {
            method: "POST",
        };
        await fetch(host, options)
            .then((response) => console.log("response: ", response))
            .catch((error) => console.error("error: ", error));
    }

    // Post to the matrix API of backend with dots as the body
    function submit() {
        console.log("submit");
        // Each element of body should have 32 entries
        let dots2 = {
            R: [],
            G: [],
            B: [],
        };
        let dots = get(stored_dots);
        // Need to re-arrange the dots to a compatible format as the body for the API
        for (let i = 0; i < 64; i++) {
            let d = dots[i].background_color;
            d = hexToRgb(d);
            d = {
                r: d.r === 0 ? 0 : (d.r - 32) / 3,
                g: d.g === 0 ? 0 : (d.g - 32) / 3,
                b: d.b === 0 ? 0 : (d.b - 32) / 3,
            };
            dots2.R.push(d.r);
            dots2.G.push(d.g);
            dots2.B.push(d.b);
        }
        fetch_matrix(dots2);
    }

    async function fetch_matrix(value) {
        let host = "http://" + location.hostname + ":8000/v1/matrix";
        let options = {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(value),
        };

        await fetch(host, options)
            .then((response) => console.log("response: ", response))
            .catch((error) => console.error("error: ", error));
    }

    // https://www.codegrepper.com/code-examples/javascript/js+rgba+to+hex
    function hexToRgb(hex) {
        var result =
            /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
        if (result) {
            var r = parseInt(result[1], 16);
            var g = parseInt(result[2], 16);
            var b = parseInt(result[3], 16);
            // return r + "," + g + "," + b; //return 23,14,45 -> reformat if needed
            return { r, g, b };
        }
        return null;
    }
</script>

<main>
    <!-- Slider R -->
    <div class="slider-container">
        <p class="slider-title">R</p>
        <p class="slider-value">{sliderR}</p>
        <input
            class="slider"
            type="range"
            min="0"
            max="63"
            id="sliderR"
            bind:value={sliderR}
            on:change={slider_changed(this.id, sliderR)}
        />
    </div>
    <!-- Slider G -->
    <div class="slider-container">
        <p class="slider-title">G</p>
        <p class="slider-value">{sliderG}</p>
        <input
            class="slider"
            type="range"
            min="0"
            max="63"
            id="sliderG"
            bind:value={sliderG}
            on:change={slider_changed(this.id, sliderG)}
        />
    </div>
    <!-- Slider B -->
    <div class="slider-container">
        <p class="slider-title">B</p>
        <p class="slider-value">{sliderB}</p>
        <input
            class="slider"
            type="range"
            min="0"
            max="63"
            id="sliderB"
            bind:value={sliderB}
            on:change={slider_changed(this.id, sliderB)}
        />
    </div>
    <br />

    <!-- Select All/None-->
    <div class="button-container">
        <p class="button-title">Select</p>
        <button class="button" id="all" on:click={select_all}>All</button>
        <button class="button" id="none" on:click={select_none}>None</button>
        <p class="padding" />
    </div>

    <!-- Effect -->
    <div class="button-container">
        <p class="button-title">Effect</p>
        <button class="button" id="random" on:click={random}>Random</button>
        <p class="padding" />
    </div>

    <!-- Submit -->
    <div class="button-container">
        <p class="button-title" />
        <button class="button" id="submit" on:click={submit}>Submit</button>
        <p class="padding" />
    </div>
</main>

<style>
    main {
        /* text-align: center; */
        /* align-items: center; */
        background-color: #fefefe;
        margin-top: 1px;
        margin-bottom: 8px;
        /* max-width: 240px; */
    }

    .slider-container {
        height: 42px;
        width: 490px;
        display: flex;
        align-items: center;
        margin-top: 2px;
        margin-bottom: 2px;
        margin-left: 64px;
        margin-right: 64px;
    }

    .slider-title {
        width: 32px;
        display: inline-block;
        text-align: end;
        margin-right: 10px;
        font-weight: 300;
        font-size: 24px;
    }

    .slider-value {
        width: 32px;
        display: inline-block;
        text-align: end;
        margin-left: 20px;
        font-weight: 300;
        font-size: 20px;
    }

    /* https://www.w3schools.com/howto/howto_js_rangeslider.asp */
    .slider {
        flex: 0 1 auto;
        width: 36%;
        margin-top: 6px;
        margin-left: 64px;
        margin-right: 2px;
        background-color: #ffffff;
        -webkit-appearance: none; /* Override default CSS styles */
        appearance: none;
        height: 20px; /* Specified height */
        background: #d3d3d3; /* Grey background */
        outline: none; /* Remove outline */
        opacity: 0.7; /* Set transparency (for mouse-over effects on hover) */
        -webkit-transition: 0.2s; /* 0.2 seconds transition on hover */
        transition: opacity 0.1s;
    }

    .slider::-webkit-slider-thumb {
        -webkit-appearance: none; /* Override default look */
        appearance: none;
        border-radius: 0.2em;
        width: 25px; /* Set a specific slider handle width */
        height: 25px; /* Slider handle height */
        background: #1111ff; /* Green background */
        cursor: pointer; /* Cursor on hover */
    }

    .button-container {
        text-align: center;
        height: 48px;
        width: 418px;
        display: flex;
        align-items: center;
        justify-content: center;
        margin-left: 32px;
        flex-direction: row;
    }

    .button-title {
        flex: 1 0 auto;
        text-align: start;
        display: inline-block;
        padding-left: 64px;
        font-weight: 300;
        font-size: 20px;
    }

    .button {
        width: 85px;
        margin-top: 10px;
        margin-left: 8px;
        margin-right: 8px;
        border-width: 2px;
        border-radius: 0.3em;
        border-color: #888888;
        font-size: 16px;
    }

    .padding {
        width: 42px;
    }

    @media (min-width: 400px) {
        main {
            max-width: none;
        }
    }
</style>
