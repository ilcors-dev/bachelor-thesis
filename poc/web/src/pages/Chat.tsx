import autoAnimate from '@formkit/auto-animate';
import { MutableRefObject, useEffect, useRef } from 'react';
import { useQuery } from 'react-query';
import { useParams } from 'react-router-dom';
import { Message } from '../components/Message';
import { MessageInput } from '../components/MessageInput';
import { MessageLoading } from '../components/MessageLoading';

export const Chat = () => {
	const { chatId } = useParams<{ chatId: string }>();

	const messagesContainer = useRef(null);
	const bottomRef = useRef(null);

	useEffect(() => {
		messagesContainer.current && autoAnimate(messagesContainer.current);
	}, [messagesContainer]);

	const { isLoading, data, error } = useQuery<Message[]>(
		['messages'],
		async () =>
			(
				await fetch('http://localhost:9007/messages', {
					method: 'GET',
				})
			).json(),
		{
			onSuccess: (messages) => {
				if (!messages || messages.length === 0) {
					return;
				}

				setTimeout(() => {
					(bottomRef.current as unknown as HTMLDivElement)?.scrollIntoView({
						behavior: 'smooth',
					});
				}, 50);
			},
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
