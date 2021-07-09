<script>
	import Controller from "./components/Controller.svelte";
	import Matrix from "./components/Matrix.svelte";
	export let name;
</script>

<svelte:head>
	<title>{name}</title>
</svelte:head>

<main>
	<h1>{name}</h1>
</main>

<Matrix />

<div>
	<Controller />
</div>

<style>
	:global(body) {
		/* this will apply to <body> */
		padding: 0;
		/* Hide scrollbars */
		overflow: hidden;
	}

	main {
		text-align: center;
		background-color: #3333ff;
		padding: 0.1em;
		margin: 0 auto;
		max-width: none;
		box-shadow: 0px 0px 0px 0px #111111;
	}

	h1 {
		/* color: #ff3e00; */
		color: #ffffff;
		text-transform: none; /* uppercase */
		font-size: 1.3em;
		font-weight: 300;
	}

	div {
		text-align: center;
	}

	@media (min-width: 400px) {
		main {
			max-width: none;
			min-width: 512px;
		}
	}
</style>
