import { ArrowRightIcon } from '@heroicons/react/20/solid';
import { useState } from 'react';
import { useMutation, useQueryClient } from 'react-query';
import { InputEmoji } from './InputEmoji';

interface Props {
	className?: string;
}

export const MessageInput = ({ className }: Props) => {
	let client = useQueryClient();
	const [message, setMessage] = useState<string>('');

	const addMessage = useMutation({
		mutationFn: async (message: string) =>
			(
				await fetch('http://localhost:8080/create_message', {
					method: 'POST',
					body: JSON.stringify({ text: message }),
				})
			).json(),
		onSuccess: () => client.invalidateQueries('messages'),
	});

	return (
		<form
			onSubmit={(event) => {
				event.preventDefault();
				addMessage.mutate(message);
			}}
			className={`${className}`}
		>
			<div className="flex space-x-4">
				<InputEmoji />
				<textarea
					required={true}
					id="message"
					rows={4}
					className="block w-full grow rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-blue-500 dark:focus:ring-blue-500"
					placeholder="Write your thoughts here..."
					onChange={(e) => setMessage(e.target.value)}
					value={message}
				></textarea>
				<button
					type="submit"
					className="mr-2 w-max shrink rounded-lg bg-blue-700 px-4 py-2.5 text-sm font-medium text-white hover:bg-blue-800 focus:outline-none focus:ring-4 focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
				>
					<ArrowRightIcon className="h-6 w-6" />
				</button>
			</div>
		</form>
	);
};
