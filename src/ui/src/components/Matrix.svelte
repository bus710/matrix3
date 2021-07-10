<script>
    import { stored_color } from "../stores";
    import { stored_dots } from "../stores";
    import { stored_select } from "../stores";
    import { stored_message } from "../stores";

    let background_color = "#000000ff"; // RGBA
    let font_color = "#ffffffff";
    let dots = [];
    let delimiter = [7, 15, 23, 31, 39, 47, 55];

    // For init value of the buttons
    stored_dots.subscribe((value) => {
        // console.log("dots: ", value);
        dots = value;
    });

    // New color from the slides applies to the selected buttons
    stored_color.subscribe((value) => {
        // console.log("color: ", value);
        let R = value.sliderR;
        let G = value.sliderG;
        let B = value.sliderB;
        // Change font color depends on the green part of background color
        if (G > 50) {
            font_color = "#000000ff";
        } else {
            font_color = "#ffffffff";
        }
        // Create background color depends on the sliders
        background_color =
            "#" +
            numHex(R * 3 + 32) +
            numHex(G * 3 + 32) +
            numHex(B * 3 + 32) +
            numHex(255);
        // Apply to the list
        dots.forEach((el) => {
            if (el.selected) {
                el.background_color = background_color;
                el.font_color = font_color;
            }
        });
        // Store to global
        stored_dots.set(dots);
    });

    // "all" => select all, "none" => select none
    stored_select.subscribe((value) => {
        // console.log("select: ", value);
        if (value === "all") {
            dots.forEach((el) => {
                el.selected = true;
                if (el.selected) {
                    el.background_color = background_color;
                    el.font_color = font_color;
                }
            });
        } else if (value === "none") {
            dots.forEach((el) => {
                el.selected = false;
            });
        }
        stored_dots.set(dots);
    });

    // When a new message comes in via websocket
    stored_message.subscribe((value) => {
        console.log("ws:", "(", value.length, ")", value);
        if (value.length > 0) {
            let res = JSON.parse(value);
            for (let i = 0; i < 64; i++) {
                let R = res.R[i];
                let G = res.G[i];
                let B = res.B[i];
                background_color =
                    "#" +
                    numHex(R * 3 + 32) +
                    numHex(G * 3 + 32) +
                    numHex(B * 3 + 32) +
                    numHex(255);
                dots[i].background_color = background_color;
                dots[i].selected = false;
            }
            stored_dots.set(dots);
            stored_color.set({ sliderR: 0, sliderG: 0, sliderB: 0, alpha: 0 });
            background_color = "#000000ff"; // RGBA
        }
    });

    // When any button gets pressed, the selected flag should be flipped
    function clicked() {
        console.log("id: ", this.id);
        console.log("id-dots: ", dots[this.id]);
        dots[this.id].selected = !dots[this.id].selected;
        dots[this.id].background_color = background_color;
        stored_dots.set(dots);
    }

    // https://stackoverflow.com/a/26784300/5021812
    function numHex(s) {
        var a = s.toString(16);
        if (a.length % 2 > 0) {
            a = "0" + a;
        }
        return a;
    }
</script>

<main>
    {#each dots as { selected, background_color, font_color }, i}
        {#if selected}
            <button
                class="buttons"
                style="color:{font_color}; background-color:{background_color}; border-color:red; border-width:2px"
                id={i}
                on:click={clicked}>{i}</button
            >
        {:else}
            <button
                class="buttons"
                style="color:{font_color}; background-color:{background_color}; border-color:grey; border-width:2px"
                id={i}
                on:click={clicked}>{i}</button
            >
        {/if}
        <!-- To add a line break per 8 buttons -->
        {#if delimiter.includes(i)}
            <br />
        {/if}
    {/each}
</main>

<style>
    main {
        height: 320px;
        text-align: center;
        background-color: #ffffff;
        padding: 0.2em;
        margin-top: 16px;
        margin-bottom: 24px;
        margin-left: 2px;
        margin-right: 20px;
        max-width: 240px;
        box-shadow: 0px 0px 0px 0px #111111;
    }

    .buttons {
        width: 36px;
        height: 32px;
        margin-top: 2px;
        margin-left: 2px;
        margin-right: 2px;
        border-radius: 0.5em;
        font-size: 14px;
        color: var(--color);
        border-width: 1px;
    }

    @media (min-width: 400px) {
        main {
            max-width: none;
        }
    }
</style>
