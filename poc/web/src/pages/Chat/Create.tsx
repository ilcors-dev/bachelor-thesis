import axios from 'axios';
import { Textarea, TextInput } from 'flowbite-react';
import { FormEvent, useState } from 'react';

export const Create = () => {
	const [data, setData] = useState({
		name: '',
		description: '',
	});

	const submit = async (e: FormEvent<HTMLFormElement>) => {
		e.preventDefault();

		try {
			const res = await axios.post('/api/chats', data);

			if (res.status === 201) {
				window.location.href = '/';
			}
		} catch (error) {
			console.error(error);
			alert('Something went wrong, please try again later.');
		}
	};

	return (
		<div className="container mx-auto pt-8">
			<div>
				<h1 className="text-4xl font-bold">Create a new chat</h1>
			</div>
			<form className="mt-8" onSubmit={submit}>
				<div className="mb-6">
					<label
						htmlFor="chat-name"
						className="mb-2 block text-sm font-medium text-gray-900 dark:text-white"
					>
						Chat name
					</label>
					<TextInput
						id="chat-name"
						placeholder="Enter a chat name"
						required={true}
						defaultValue={''}
						onChange={(e) => setData({ ...data, name: e.target.value })}
					/>
				</div>
				<div className="mb-6">
					<label
						htmlFor="chat-description"
						className="mb-2 block text-sm font-medium text-gray-900 dark:text-white"
					>
						Notes (optional)
					</label>
					<Textarea
						id="chat-description"
						placeholder="Enter a chat description"
						rows={4}
						defaultValue={''}
						onChange={(e) => setData({ ...data, description: e.target.value })}
					/>
				</div>

				<button
					type="submit"
					className="w-full rounded-lg bg-blue-700 px-4 py-2.5 text-sm font-medium text-white hover:bg-blue-800 focus:outline-none focus:ring-4 focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800 sm:w-auto"
				>
					Submit
				</button>
			</form>
		</div>
	);
};
