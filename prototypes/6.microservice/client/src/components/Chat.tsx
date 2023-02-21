import autoAnimate from '@formkit/auto-animate';
import { MutableRefObject, useEffect, useRef } from 'react';
import { useQuery } from 'react-query';
import { Message } from './Message';
import { MessageInput } from './MessageInput';
import { MessageLoading } from './MessageLoading';

export const Chat = () => {
	const messagesContainer = useRef(null);
	const bottomRef = useRef(null);

	useEffect(() => {
		messagesContainer.current && autoAnimate(messagesContainer.current);
	}, [messagesContainer]);

	const { isLoading, data, error } = useQuery<Message[]>(
		['messages'],
		async () =>
			(
				await fetch('http://localhost:8080/messages', {
					method: 'GET',
				})
			).json(),
		{
			onSuccess: () =>
				bottomRef.current?.scrollIntoView({ behavior: 'smooth' }),
		}
	);

	return (
		<div className="flex flex-col justify-between">
			<div>
				{isLoading && <MessageLoading />}
				<ul ref={messagesContainer} className="h-[70vh] overflow-x-auto">
					{data &&
						data.map((message) => (
							<Message key={message.id} className="my-4" message={message} />
						))}
					<div ref={bottomRef}></div>
				</ul>
			</div>
			<MessageInput className="mt-4 w-full" />
		</div>
	);
};
