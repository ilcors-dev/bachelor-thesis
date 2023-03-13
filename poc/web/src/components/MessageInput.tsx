import { ArrowRightIcon } from '@heroicons/react/20/solid';
import axios from 'axios';
import { Textarea } from 'flowbite-react';
import { useState } from 'react';
import { useMutation, useQueryClient } from 'react-query';
import { useParams } from 'react-router-dom';
import { InputEmoji } from './InputEmoji';

interface Props {
	className?: string;
}

export const MessageInput = ({ className }: Props) => {
	const { chatId } = useParams();

	let client = useQueryClient();
	const [message, setMessage] = useState<string>('');

	const addMessage = useMutation({
		mutationFn: async (message: string) => {
			const response = await axios.post('/api/messages', {
				chat_id: +(chatId ?? 0),
				text: message,
			});

			return response.data as Message[];
		},
		onSuccess: () => client.invalidateQueries('messages'),
	});

	return (
		<form
			onSubmit={(event) => {
				event.preventDefault();
				addMessage.mutate(message);
				setMessage('');
			}}
			className={`${className}`}
		>
			<div className="flex space-x-4">
				{/* <InputEmoji
					handleEmojiClick={(emoji) => setMessage(message + emoji.emoji)}
				/> */}
				<Textarea
					id="comment"
					placeholder="Write your thoughts here..."
					required={true}
					rows={4}
					value={message}
					onChange={(e) => setMessage(e.target.value)}
				/>
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
