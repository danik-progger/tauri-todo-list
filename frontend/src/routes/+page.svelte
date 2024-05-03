<script lang="ts">
    import type { PageData } from "./$types";
	import { invalidateAll } from "$app/navigation";

	export let data: PageData;

	async function deleteTodo(id: number) {
		await fetch(`http://0.0.0.0:8000/delete/${id}`, {
            method: "POST"
        });
		invalidateAll();
	}

	async function markAsDone(todo) {
		await fetch(`http://0.0.0.0:8000/update?id=${todo.id}&description=${todo.description}&done=${todo.done}`);
	}
</script>




<div class="container h-full p-10 mx-auto flex flex-col justify-center items-center">
	<h1 class="h1 text-center">Todos</h1>

	<form class="w-6/12 mx-auto my-10" action="http://0.0.0.0:8000/create" method="post">
		<input class="input w-full" type="text" name="description" placeholder="Type new todo and hit Enter">
	</form>

	{#each data.todos as todo}
	<div class={"flex flex-row m-1 w-6/12 justify-between items-center" + (todo.done && "opacity-50")}>
		<div class="flex flex-row m-1 items-center gap-5">
			<input class="checkbox" type="checkbox" bind:checked={todo.done} on:change={() => markAsDone(todo)}>
			<p class={todo.done && "line-through"}>  {todo.description}  </p>
		</div>
		<button class="btn variant-filled-primary" on:click={() => deleteTodo(todo.id)}>X</button>
	</div>

	{/each}
</div>
