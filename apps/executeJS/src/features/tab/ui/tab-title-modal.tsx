import { Tab, usePlaygroundStore } from '@/features/playground';
import { Cross1Icon } from '@radix-ui/react-icons';
import { Controller, SubmitHandler, useForm } from 'react-hook-form';

interface TabTitleModalProps {
  tab: Tab;
  onClose: () => void;
}

export const TabTitleModal: React.FC<TabTitleModalProps> = ({
  tab,
  onClose,
}) => {
  const { id, title } = tab;

  const { setTabTitle } = usePlaygroundStore();

  const {
    control,
    handleSubmit,
    formState: { isValid },
  } = useForm<Pick<Tab, 'id' | 'title'>>({
    defaultValues: {
      id,
      title,
    },
  });

  const handleChangeTabTitle: SubmitHandler<Pick<Tab, 'id' | 'title'>> = ({
    id,
    title,
  }) => {
    const trimmedTitle = title.trim();

    setTabTitle({ tabId: id, title: trimmedTitle });
    onClose();
  };

  return (
    <div className="fixed top-0 right-0 bottom-0 left-0 z-10 flex items-center justify-center">
      <div
        className="absolute top-0 right-0 bottom-0 left-0 bg-white opacity-20"
        onClick={onClose}
      />

      <div className="absolute flex flex-col p-6 border border-black rounded-2xl bg-gray-950 shadow-[2px_4px_4px_rgba(0,0,0,0.3)]">
        <div className="flex justify-between items-center">
          <strong className="text-lg">Change tab title</strong>
          <button type="button" onClick={onClose} className="cursor-pointer">
            <Cross1Icon />
          </button>
        </div>

        <Controller
          control={control}
          name="title"
          rules={{
            required: true,
            validate: (value) => value.trim().length > 0,
          }}
          render={({ field: { value, onChange } }) => {
            return (
              <input
                type="text"
                autoFocus
                value={value}
                onChange={onChange}
                className="py-1 px-2 mt-4 mb-3 rounded-md bg-gray-900"
              />
            );
          }}
        />

        <button
          type="submit"
          disabled={!isValid}
          onClick={handleSubmit(handleChangeTabTitle)}
          className="p-1 rounded-md bg-blue-600 cursor-pointer hover:bg-blue-700 disabled:bg-slate-700 disabled:cursor-not-allowed"
        >
          Save
        </button>
      </div>
    </div>
  );
};
