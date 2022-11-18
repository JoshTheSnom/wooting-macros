import {
  Button,
  Input,
  Modal,
  ModalBody,
  ModalCloseButton,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalOverlay
} from '@chakra-ui/react'
import { BaseSyntheticEvent, useState } from 'react'
import { useApplicationContext } from '../../contexts/applicationContext'
import { useSelectedCollection } from '../../contexts/selectors'
import { Collection } from '../../types'
import { updateBackendConfig } from '../../utils'

type Props = {
  isRenaming: boolean
  isOpen: boolean
  onClose: () => void
}

const CollectionModal = ({ isRenaming, isOpen, onClose }: Props) => {
  const [collectionName, setCollectionName] = useState('')
  const [isNameUsable, setIsNameUsable] = useState(true)

  const { collections, changeSelectedCollectionIndex } = useApplicationContext()
  const collection: Collection = useSelectedCollection()

  const onModalClose = () => {
    if (isRenaming) {
      collection.name = collectionName
    } else {
      collections.push({
        active: true,
        icon: 'i',
        macros: [],
        name: collectionName
      })
    }
    changeSelectedCollectionIndex(collections.length - 1)
    onClose()
    updateBackendConfig(collections)
  }

  // unsure if BaseSyntheticEvent is the right type for this
  const onCollectionNameChange = (event: BaseSyntheticEvent) => {
    let newName: string = event.target.value
    newName = newName.trim()

    setCollectionName(newName)
    for (let i = 0; i < collections.length; i++) {
      const collection = collections[i]
      if (collection.name.toUpperCase() === newName.toUpperCase()) {
        setIsNameUsable(false)
        return
      }
    }
    setIsNameUsable(true)
  }

  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <ModalOverlay />
      <ModalContent>
        <ModalHeader>
          {isRenaming ? 'Changed your mind?' : 'Give it a unique name!'}
        </ModalHeader>
        <ModalCloseButton />
        <ModalBody>
          <Input
            variant="flushed"
            isRequired
            isInvalid={!isNameUsable}
            onChange={onCollectionNameChange}
            placeholder={isRenaming ? collection.name : 'Collection Name'}
          />
        </ModalBody>
        <ModalFooter>
          <Button mr={3} onClick={onClose}>
            Close
          </Button>
          <Button colorScheme="yellow" onClick={onModalClose}>
            {isRenaming ? 'Rename' : 'Create'}
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  )
}

export default CollectionModal